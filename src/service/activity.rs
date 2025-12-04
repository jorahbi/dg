use crate::model::transactions::{OrderStatus, OrderType, TransactionsBuilder};
use crate::repository::{TransactionsRepo, UserRepo};
use crate::utils::time_zone::TimeZone;
use crate::{error::Result, state::AppState, AppError};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SystemActivityWelcomeBonus {
    pub unit: String,
    pub amount: i64,
}
pub struct ActivityService {
    db: sqlx::MySqlPool,
}

impl ActivityService {
    pub fn new(state: &AppState) -> Self {
        Self {
            db: (*state.db).clone(),
        }
    }

    // 领取新手福利
    pub async fn welcome_bonus(
        &self,
        user_id: u64,
        bonus: &SystemActivityWelcomeBonus,
    ) -> Result<()> {
        if bonus.amount <= 0 {
            return Err(AppError::NotFound("Newcomer benefit amount setting error".to_string()));
        }
        let amount = Decimal::from(bonus.amount);
        let curr_time = TimeZone::Beijing.get_time();
        let tran = TransactionsBuilder::default()
            .user_id(user_id)
            .types(OrderType::Welcome.to_string())
            .amount(amount)
            .status(OrderStatus::Completed.to_string())
            .created_at(curr_time)
            .updated_at(curr_time)
            .build()
            .map_err(|e| AppError::Internal(format!("build transaction:{}", e.to_string())))?;
        let mut tx = self.db.begin().await?;
        let res = UserRepo::tx_update_assets(&mut *tx, user_id, &amount, &Decimal::ZERO).await;
        if let Err(err) = res {
            tx.rollback().await?;
            return Err(AppError::Internal(format!("Failed to update user assets: {}", err)));
        }
        let res = TransactionsRepo::tx_create(&mut *tx, &vec![tran]).await;
        if let Err(err) = res {
            tx.rollback().await?;
            return Err(AppError::Internal(format!("Failed to insert benefit claim record: {}", err)));
        }

        tx.commit().await?;
        Ok(())
    }
}
