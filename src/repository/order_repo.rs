use crate::error::Result;
use crate::model::order::Order;
use crate::AppError;
use rust_decimal::Decimal;
use sqlx::{MySql, MySqlConnection, Pool};

pub struct OrderRepo;

impl OrderRepo {
    /// 在事务中创建购买订单
    pub async fn create_order_in_tx(
        tx: &mut MySqlConnection,
        user_id: u64,
        order_no: &str,
        power_package_id: u64,
        quantity: u32,
        amount: Decimal,
        asset_pay: Decimal,
        coin_pay: Decimal,
        blockchain_type: &str,
        addr: &str,
        state: i8,
    ) -> Result<u64> {
        // 生成支付接收地址（示例地址，实际应该从配置或服务获取）
        let result = sqlx::query!(
            r#"
            INSERT INTO orders (
                order_id, user_id, power_package_id,
                quantity, amount, blockchain_type, blockchain_address,
                status, asset_pay, coin_pay
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            order_no,
            user_id,
            power_package_id,
            quantity,
            amount,
            blockchain_type,
            addr,
            state,
            asset_pay,
            coin_pay
        )
        .execute(&mut *tx)
        .await?;

        Ok(result.last_insert_id())
    }

    /// 根据订单ID获取订单详情
    pub async fn get_order_by_id(pool: &Pool<MySql>, order_id: &str) -> Result<Option<Order>> {
        let order = sqlx::query_as!(
            Order,
            r#"
            SELECT id, order_id as "order_id: String", user_id, power_package_id, quantity,asset_pay,coin_pay,
                   amount, blockchain_type as "blockchain_type: String", blockchain_address  as "blockchain_address: String",
                   transaction_hash as "transaction_hash: String",
                   status, created_at, updated_at
            FROM orders
            WHERE order_id = ?
            "#,
            order_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(order)
    }

    /// 根据订单ID获取订单详情
    pub async fn get_order_by_id_and_user_id(
        pool: &Pool<MySql>,
        order_id: &str,
        user_id: u64,
    ) -> Result<Order> {
        let order = sqlx::query_as!(
            Order,
            r#"
            SELECT id, order_id as "order_id: String", user_id, power_package_id, quantity,asset_pay,coin_pay,
                   amount, blockchain_type as "blockchain_type: String", blockchain_address as "blockchain_address: String",
                   transaction_hash as "transaction_hash: String",
                   status, created_at, updated_at
            FROM orders
            WHERE order_id = ? AND user_id = ?
            "#,
            order_id,
            user_id
        )
        .fetch_optional(pool)
        .await?;
        match order {
            Some(order) => Ok(order),
            None => Err(AppError::NotFound(format!(
                "Order with id {} not found",
                order_id
            ))),
        }
    }

    /// 根据订单ID更新订单状态
    pub async fn update_order_status(
        pool: &Pool<MySql>,
        order_id: &str,
        status: i8,
    ) -> Result<Option<Order>> {
        let now = chrono::Utc::now();
        sqlx::query!(
            r#"
            UPDATE orders
            SET status = ?,
                updated_at = ?
            WHERE order_id = ?
            "#,
            status,
            now,
            order_id
        )
        .execute(pool)
        .await?;

        Self::get_order_by_id(pool, order_id).await
    }

    /// 在事务中更新订单状态和对应的算力记录状态
    pub async fn update_order_and_power_status_in_tx(
        tx: &mut MySqlConnection,
        order_id: &str,
        new_order_status: i8,
        new_power_status: i8,
    ) -> Result<()> {
        let now = chrono::Utc::now();

        // 1. 更新订单状态
        sqlx::query!(
            r#"
            UPDATE orders
            SET status = ?,
                updated_at = ?
            WHERE order_id = ?
            "#,
            new_order_status,
            now,
            order_id
        )
        .execute(&mut *tx)
        .await?;
        // 3. 更新对应的算力记录状态
        sqlx::query!(
            r#"
            UPDATE user_power SET status = ?, updated_at = ? WHERE order_id = ?
            "#,
            new_power_status,
            now,
            order_id
        )
        .execute(&mut *tx)
        .await?;

        Ok(())
    }

    pub async fn tx_update_order(
        tx: &mut MySqlConnection,
        order: &Order,
        new_order_status: i8,
        new_power_status: i8,
    ) -> Result<()> {
        Self::update_order_and_power_status_in_tx(
            tx,
            &order.order_id,
            new_order_status,
            new_power_status,
        )
        .await?;
        Ok(())
    }
}
