use crate::model::system_config::{ConfigLevel, SystemConfigType};
use crate::model::transactions::{OrderStatus, OrderType, TransactionsBuilder};
use crate::model::{
    PowerPackage, User, ORDER_STATUS_CANCELLED, ORDER_STATUS_PAID, ORDER_STATUS_PENDING,
    USER_POWER_RECORD_STATUS_ACTIVE,
};
use crate::repository::power_repo::PowerRepo;
use crate::repository::{InviteRepo, SystemConfigRepo, TransactionsRepo, UserRepo};
use crate::utils::gen::generate_no;
use crate::utils::time_zone::TimeZone;
use crate::{error::Result, repository::OrderRepo, state::AppState, AppError};
use bigdecimal::ToPrimitive;
use rust_decimal::Decimal;
use sqlx::MySqlConnection;
use std::collections::HashMap;

pub struct OrderService {
    db: sqlx::MySqlPool,
}

impl OrderService {
    pub fn new(state: &AppState) -> Self {
        Self {
            db: (*state.db).clone(),
        }
    }

    /// 创建订单（使用事务）
    pub async fn create_order(
        &self,
        user: &User,
        power: &PowerPackage,
        chain_type: &str,
        addr: &str,
    ) -> Result<(u64, String)> {
        let mut tx = self.db.begin().await?;
        let order = self
            .save_order(&mut tx, power, user.total_assets, user.id, chain_type, addr)
            .await;
        let Ok(order) = order else {
            tx.rollback().await?;
            return Err(AppError::Internal("Order creation failed".to_string()));
        };
        // 提交事务
        match tx.commit().await {
            Ok(_) => Ok(order), // 提交成功
            Err(e) => Err(AppError::Internal(format!("Transaction commit failed: {}", e))),
        }
    }

    /// 订单升级（使用事务）
    pub async fn upgrade_order(
        &self,
        user: &User,
        power: &PowerPackage,
        total_assets: Decimal,
        chain_type: &str,
        addr: &str,
        old_id: u64,
    ) -> Result<(u64, String)> {
        let mut tx = self.db.begin().await?;
        let res = PowerRepo::upgrade_user_power_record(&mut tx, user.id, old_id).await;
        if let Err(err) = res {
            tx.rollback().await?;
            return Err(AppError::Internal(format!(
                "Failed to set original power status during upgrade: {}",
                err
            )));
        };
        let order = self
            .save_order(&mut tx, power, total_assets, user.id, chain_type, addr)
            .await;

        let order = match order {
            Ok(order) => order,
            Err(err) => {
                tx.rollback().await?;
                return Err(AppError::Internal(format!("Order creation failed: {}", err)));
            }
        };

        //删除之前的算力
        // 提交事务
        match tx.commit().await {
            Ok(_) => Ok(order), // 提交成功
            Err(e) => Err(AppError::Internal(format!("Transaction commit failed: {}", e))),
        }
    }

    pub async fn save_order(
        &self,
        tx: &mut MySqlConnection,
        power: &PowerPackage,
        total_assets: Decimal,
        user_id: u64,
        chain_type: &str,
        addr: &str,
    ) -> Result<(u64, String)> {
        // 计算订单总金额
        let Some(package_amount) = power.amount.to_f64() else {
            return Err(AppError::Internal(format!("Amount conversion failed for power ID {}", power.id)));
        };
        let total_amount = Decimal::new((package_amount * 1.0f64 * 100.0).round() as i64, 2);
        let mut asset_pay = Decimal::new(0, 2);
        let mut coin_pay = Decimal::new(0, 2);
        let mut new_total_assets = Decimal::new(0, 2);
        let zero = Decimal::new(0, 0);
        if total_assets > zero {
            if total_assets >= total_amount {
                asset_pay = total_amount;
                new_total_assets = total_assets - total_amount;
            } else {
                asset_pay = total_assets;
                coin_pay = total_amount - total_assets;
                new_total_assets = zero;
            }
        } else {
            coin_pay = total_amount
        }

        // 开始事务

        let state = if coin_pay == zero {
            ORDER_STATUS_PAID
        } else {
            ORDER_STATUS_PENDING
        };
        // 创建订单
        let order_no = generate_no("O");
        let order_id = match OrderRepo::create_order_in_tx(
            tx,
            user_id,
            &order_no,
            power.id,
            1,
            total_amount,
            asset_pay,
            coin_pay,
            chain_type,
            addr,
            state,
        )
        .await
        {
            Ok(order) => order,
            Err(e) => return Err(e),
        };

        // 为用户创建算力记录（每个购买数量创建一条记录）
        let power_amount = power.amount;
        let current_time = TimeZone::Beijing.get_time();
        match PowerRepo::create_user_power_record(
            tx,
            user_id,
            power.id,
            &order_no,
            0, // 0 表示购买
            &power_amount,
            state as i16,
            power.lv,
            &power.daily_yield_percentage,
        )
        .await
        {
            Ok(_) => {} // 继续下一个
            Err(e) => return Err(e),
        }

        if total_assets != new_total_assets {
            // 总资产变化
            let res =
                UserRepo::update_user_assets_in_tx(tx, user_id, new_total_assets, total_assets)
                    .await;
            if let Err(err) = res {
                return Err(err);
            }
            let tran = TransactionsBuilder::default()
                .user_id(user_id)
                .types(OrderType::Purchase.to_string())
                .amount(Decimal::ZERO - asset_pay)
                .status(OrderStatus::Completed.to_string())
                .created_at(current_time)
                .updated_at(current_time)
                .build()
                .map_err(|e| AppError::Internal(format!("build transaction:{}", e.to_string())))?;
            let res = TransactionsRepo::tx_create(tx, &vec![tran]).await;
            if let Err(err) = res {
                return Err(err);
            }
        }

        Ok((order_id, order_no))
    }

    /// 取消订单
    pub async fn cancel_order(&self, user_id: u64, order_id: &str) -> Result<()> {
        // 获取订单信息
        let order = OrderRepo::get_order_by_id_and_user_id(&self.db, order_id, user_id).await?;

        // 验证订单状态（只有待支付的订单才能取消）
        if order.status != ORDER_STATUS_PENDING {
            return Err(AppError::Business("Only pending orders can be cancelled".to_string()));
        }
        let user = UserRepo::find_by_id(&self.db, order.user_id).await?;
        let mut tx = self.db.begin().await?;
        let update = OrderRepo::tx_update_order(
            &mut tx,
            &order,
            ORDER_STATUS_CANCELLED,
            ORDER_STATUS_CANCELLED,
        )
        .await;
        if let Err(err) = update {
            tx.rollback().await?;
            return Err(AppError::Internal(format!(
                "Failed to update order status: {}",
                err.to_string()
            )));
        }
        if order.asset_pay > Decimal::new(0, 2) {
            let asset = UserRepo::update_user_assets_in_tx(
                &mut tx,
                order.user_id,
                user.total_assets + order.asset_pay,
                user.total_assets,
            )
            .await;
            if let Err(err) = asset {
                tx.rollback().await?;
                return Err(AppError::Internal(format!(
                    "Failed to update user assets while updating order status: {}",
                    err.to_string()
                )));
            }
            let current_time = TimeZone::Beijing.get_time();
            let tran = TransactionsBuilder::default()
                .user_id(order.user_id)
                .types(OrderType::CancelPurchase.to_string())
                .amount(order.asset_pay)
                .status(OrderStatus::Completed.to_string())
                .created_at(current_time)
                .updated_at(current_time)
                .build()
                .map_err(|e| AppError::Internal(format!("build transaction:{}", e.to_string())))?;
            let res = TransactionsRepo::tx_create(&mut tx, &vec![tran]).await;
            if let Err(err) = res {
                tx.rollback().await?;
                return Err(err);
            }
        }

        // 提交事务
        tx.commit().await?;

        Ok(())
    }

    /// 订单支付完成
    pub async fn paid_order(&self, user_id: u64, order_id: &str) -> Result<()> {
        // 获取订单信息
        let order = OrderRepo::get_order_by_id_and_user_id(&self.db, order_id, user_id).await?;

        // 验证订单状态（只有待支付的订单才能付款）
        if order.status != ORDER_STATUS_PENDING {
            return Err(AppError::Business("Only pending orders can be paid".to_string()));
        }
        let user = UserRepo::find_by_id(&self.db, order.user_id).await?;
        let config = SystemConfigRepo::get_config_by_key(
            &self.db,
            &SystemConfigType::UpgradeProgress.to_string(),
        )
        .await?;
        let mut lvs: Vec<ConfigLevel> = serde_json::from_str(&config.config_value)?;
        lvs.sort_by(|lv1, lv2| lv2.cmp(lv1));
        let mut curr_lv = user.user_level;
        for lv in lvs {
            let points = Decimal::from(user.upgrade_progress) + order.coin_pay;
            if points >= Decimal::from(lv.recharge) {
                curr_lv = lv.lv;
            }
        }

        let mut tx = self.db.begin().await?;
        let res = UserRepo::tx_update_lv(&mut tx, user.id, curr_lv, &order.coin_pay).await;
        if let Err(err) = res {
            tx.rollback().await?;
            return Err(AppError::Internal(format!(
                "Failed to update user level information: {}",
                err.to_string()
            )));
        }

        let update = OrderRepo::tx_update_order(
            &mut tx,
            &order,
            ORDER_STATUS_PAID,
            USER_POWER_RECORD_STATUS_ACTIVE as i8,
        )
        .await;
        if let Err(err) = update {
            tx.rollback().await?;
            return Err(AppError::Internal(format!(
                "Failed to update order status: {}",
                err.to_string()
            )));
        }

        let invite = InviteRepo::invite_chain(
            &mut tx,
            user.inviter_id,
            user.parent_inviter_id,
            &order.coin_pay,
        )
        .await;
        let mut trans = match invite {
            Ok(invite) => invite,
            Err(err) => {
                return Err(AppError::Internal(format!(
                    "Recharge sharing failed: {}",
                    err.to_string()
                )));
            }
        };
        if order.coin_pay > Decimal::new(0, 2) {
            let current_time = TimeZone::Beijing.get_time();
            let tran = TransactionsBuilder::default()
                .user_id(order.user_id)
                .types(OrderType::Purchase.to_string())
                .status(OrderStatus::Completed.to_string())
                .created_at(current_time)
                .updated_at(current_time)
                .amount(order.coin_pay)
                .build()
                .map_err(|e| AppError::Internal(format!("build transaction:{}", e.to_string())))?;
            trans.push(tran);
        }
        let res = TransactionsRepo::tx_create(&mut tx, &trans).await;
        if let Err(err) = res {
            tx.rollback().await?;
            return Err(err);
        }

        // 提交事务
        tx.commit().await?;
        Ok(())
    }
}
