use crate::{
    error::Result, model::power::PowerPackage, repository::power_repo::PowerRepo,
    schema::power::PowerPackageResponse, state::AppState,
};

pub struct PowerService {
    db: sqlx::MySqlPool,
}

impl PowerService {
    pub fn new(state: &AppState) -> Self {
        Self {
            db: (*state.db).clone(),
        }
    }

    /// 根据ID获取算力包详情
    pub async fn get_power_record_by_id(&self, power_id: u64) -> Result<Option<PowerPackage>> {
        PowerRepo::get_power_record_by_id(&self.db, power_id).await
    }

    pub async fn get_packages(&self) -> Result<Vec<PowerPackageResponse>> {
        // 暂时返回空实现
        Ok(vec![])
    }

    pub async fn get_package_detail(&self, _power_id: i64) -> Result<PowerPackageResponse> {
        // 暂时返回默认实现
        Ok(PowerPackageResponse {
            id: 1,
            task_type: "AI Intelligent Computing".to_string(),
            required_level: 1,
            earnings_percent: 5.8,
            amount: 150.00,
            currency: "USDT".to_string(),
            description: "Providing stable and efficient AI computing services".to_string(),
            duration: "30 days".to_string(),
            features: vec![
                "High-performance AI computing".to_string(),
                "Stable earnings".to_string(),
                "Automatic reinvestment".to_string(),
            ],
            daily_earnings: 8.70,
            total_earnings: 261.00,
        })
    }
}
