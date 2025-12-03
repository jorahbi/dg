use crate::model::transactions::{OrderStatus, OrderType, Transactions, TransactionsBuilder};
use crate::repository::UserRepo;
use crate::utils::gen::generate_no;
use crate::utils::time_zone::TimeZone;
use crate::{error::Result, state::AppState, AppError};
use rust_decimal::Decimal;
use sqlx::MySqlConnection;

pub struct InviteRepo;

impl InviteRepo {
    pub async fn simple_method(_state: &AppState) -> Result<()> {
        // 暂时空实现，避免SQLX编译错误
        // TODO: 实现真正的数据库操作
        Ok(())
    }

    pub async fn invite_chain(
        tx: &mut MySqlConnection,
        invite_id: u64,
        parent_invite_id: u64,
        amount: &Decimal,
    ) -> Result<Vec<Transactions>> {
        let mut invites: Vec<(u64, &Decimal)> = Vec::new();
        if invite_id > 0 {
            let invite = (amount * Decimal::from(10)) / Decimal::from(100);
            UserRepo::tx_update_assets(tx, invite_id, &invite, &Decimal::ZERO).await?;
            invites.push((invite_id, amount));
        }
        if parent_invite_id > 0 {
            let parent_invite = (amount * Decimal::from(5)) / Decimal::from(100);
            UserRepo::tx_update_assets(tx, parent_invite_id, &parent_invite, &Decimal::ZERO)
                .await?;
            invites.push((parent_invite_id, amount));
        }

        let mut trans: Vec<Transactions> = vec![];
        let current_time = TimeZone::Beijing.get_time();
        for (id, amount) in invites {
            let tran = TransactionsBuilder::default()
                .user_id(id)
                .transaction_id(generate_no("T"))
                .types(OrderType::Purchase.to_string())
                .status(OrderStatus::Completed.to_string())
                .created_at(current_time)
                .updated_at(current_time)
                .amount(amount.clone())
                .build()
                .map_err(|e| AppError::Internal(format!("build transaction:{}", e.to_string())))?;
            trans.push(tran);
        }

        // 暂时空实现，避免SQLX编译错误
        // TODO: 发送站内信
        Ok(trans)
    }
}
