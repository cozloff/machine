use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct WorldBankDataPoint {
    pub country: WorldBankCountry,
    pub countryiso3code: Option<String>,
    pub date: String,
    pub indicator: Option<WorldBankIndicator>,
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(super) struct WorldBankCountry {
    pub id: Option<String>,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub(super) struct WorldBankIndicator {
    pub id: Option<String>,
}
