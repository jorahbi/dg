use crate::model::transactions::{OrderStatus, OrderType};
use crate::utils::time_zone::TimeZone;
use crate::{error::Result, model::transactions::Transactions};
use sqlx::QueryBuilder;
use sqlx::{MySqlConnection, MySqlPool};
use time::OffsetDateTime;

pub struct TransactionsRepo;

impl TransactionsRepo {
    /// 创建新的交易记录
    pub async fn tx_create(
        pool: &mut MySqlConnection,
        transactions: &Vec<Transactions>,
    ) -> Result<u64> {
        let curr_time = TimeZone::Beijing.get_time();
        let mut qb = QueryBuilder::<sqlx::MySql>::new(
            r#"
            INSERT INTO transactions (
                user_id, transaction_id, types, from_currency, to_currency,
                amount, fee, exchange_rate, blockchain_type,
                from_address, to_address, description, metadata,status, created_at, updated_at
            )
            "#,
        );
        qb.push_values(transactions, |mut b, transaction| {
            b.push_bind(&transaction.user_id)
                .push_bind(&transaction.transaction_id)
                .push_bind(transaction.types.to_string())
                .push_bind(&transaction.from_currency)
                .push_bind(&transaction.to_currency)
                .push_bind(&transaction.amount)
                .push_bind(&transaction.fee)
                .push_bind(&transaction.exchange_rate)
                .push_bind(&transaction.blockchain_type)
                .push_bind(&transaction.from_address)
                .push_bind(&transaction.to_address)
                .push_bind(&transaction.description)
                .push_bind(&transaction.metadata)
                .push_bind(transaction.status.to_string())
                .push_bind(&curr_time)
                .push_bind(&curr_time);
        });
        let query = qb.build();

        let result = query.execute(pool).await?;

        Ok(result.last_insert_id())
    }

    /// 根据ID查找交易记录
    pub async fn find_by_id(pool: &MySqlPool, id: u64) -> Result<Option<Transactions>> {
        let transaction = sqlx::query_as!(
            Transactions,
            r#"
            SELECT
                id, user_id, transaction_id as "transaction_id: String",
                types as "types: String", from_currency as "from_currency: String",
                to_currency as "to_currency: String", amount, fee, exchange_rate,
                status as "status: String", blockchain_type as "blockchain_type: String",
                from_address as "from_address: String", to_address as "to_address: String",
                description as "description: String", completed_at, created_at, updated_at,
                metadata
            FROM transactions
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(pool)
        .await?;

        Ok(transaction)
        // Ok(None)
    }

    /// 根据交易ID查找交易记录
    pub async fn find_by_transaction_id(
        pool: &MySqlPool,
        transaction_id: &str,
    ) -> Result<Option<Transactions>> {
        let transaction = sqlx::query_as!(
            Transactions,
            r#"
            SELECT
                id, user_id, transaction_id as "transaction_id: String",
                types as "types: String", from_currency as "from_currency: String",
                to_currency as "to_currency: String", amount, fee, exchange_rate,
                status as "status: String", blockchain_type as "blockchain_type: String",
                from_address as "from_address: String", to_address as "to_address: String",
                description as "description: String", completed_at, created_at, updated_at,
                metadata
            FROM transactions
            WHERE transaction_id = ?
            "#,
            transaction_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(transaction)
        // Ok(None)
    }

    /// 更新交易状态
    pub async fn update_status(
        pool: &MySqlPool,
        id: u64,
        status: OrderStatus,
        completed_at: Option<OffsetDateTime>,
    ) -> Result<u64> {
        let result = sqlx::query!(
            r#"
            UPDATE transactions
            SET status = ?, completed_at = ?, updated_at = ?
            WHERE id = ?
            "#,
            status.to_string(),
            completed_at,
            TimeZone::Beijing.get_time(),
            id
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// 根据用户ID查询交易列表
    pub async fn find_by_user_id(
        pool: &MySqlPool,
        user_id: u64,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Transactions>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let transactions = sqlx::query_as!(
            Transactions,
            r#"
            SELECT
                id, user_id, transaction_id as "transaction_id: String",
                types as "types: String", from_currency as "from_currency: String",
                to_currency as "to_currency: String", amount, fee, exchange_rate,
                status as "status: String", blockchain_type as "blockchain_type: String",
                from_address as "from_address: String", to_address as "to_address: String",
                description as "description: String", completed_at, created_at, updated_at,
                metadata
            FROM transactions
            WHERE user_id = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(transactions)
    }

    /// 根据用户ID和交易类型查询交易列表
    pub async fn find_by_user_id_and_type(
        pool: &MySqlPool,
        user_id: u64,
        types: OrderType,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Transactions>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let transactions = sqlx::query_as!(
            Transactions,
            r#"
            SELECT
                id, user_id, transaction_id as "transaction_id: String",
                types as "types: String", from_currency as "from_currency: String",
                to_currency as "to_currency: String", amount, fee, exchange_rate,
                status as "status: String", blockchain_type as "blockchain_type: String",
                from_address as "from_address: String", to_address as "to_address: String",
                description as "description: String", completed_at, created_at, updated_at,
                metadata
            FROM transactions
            WHERE user_id = ? AND types = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            user_id,
            types.to_string(),
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(transactions)
    }

    /// 根据状态查询交易列表
    pub async fn find_by_status(
        pool: &MySqlPool,
        status: OrderStatus,
        limit: Option<u32>,
        offset: Option<u32>,
    ) -> Result<Vec<Transactions>> {
        let limit = limit.unwrap_or(20);
        let offset = offset.unwrap_or(0);

        let transactions = sqlx::query_as!(
            Transactions,
            r#"
            SELECT
                id, user_id, transaction_id as "transaction_id: String",
                types as "types: String", from_currency as "from_currency: String",
                to_currency as "to_currency: String", amount, fee, exchange_rate,
                status as "status: String", blockchain_type as "blockchain_type: String",
                from_address as "from_address: String", to_address as "to_address: String",
                description as "description: String", completed_at, created_at, updated_at,
                metadata
            FROM transactions
            WHERE status = ?
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
            status.to_string(),
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;

        Ok(transactions)
    }

    /// 统计用户交易数量
    pub async fn count_by_user_id(pool: &MySqlPool, user_id: u64) -> Result<u64> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM transactions WHERE user_id = ?",
            user_id
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count as u64)
    }

    /// 统计指定状态的交易数量
    pub async fn count_by_status(pool: &MySqlPool, status: OrderStatus) -> Result<u64> {
        let result = sqlx::query!(
            "SELECT COUNT(*) as count FROM transactions WHERE status = ?",
            status.to_string()
        )
        .fetch_one(pool)
        .await?;

        Ok(result.count as u64)
    }
}
