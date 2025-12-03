use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChartDataRequest {
    pub time_range: Option<String>,
    pub chart_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PowerChartData {
    pub labels: Vec<String>,
    pub earnings: Vec<f64>,
    pub hashrate: Vec<f64>,
    pub active_power: f64,
}