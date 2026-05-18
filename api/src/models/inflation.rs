use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct InflationRateRecord {
    pub country_code: String,
    pub period: String,
    pub value: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct InflationRateSnapshot {
    pub country_code: String,
    pub period: String,
    pub annual_percent_change: Option<f64>,
}

impl InflationRateSnapshot {
    pub fn new(country_code: String, period: String) -> Self {
        Self {
            country_code,
            period,
            annual_percent_change: None,
        }
    }
}
