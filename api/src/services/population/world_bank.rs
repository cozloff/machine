use super::PopulationServiceError;

mod dto;

pub(in crate::services::population) use dto::POPULATION_INDICATORS;
use dto::WorldBankDataPoint;

pub(super) struct WorldBankClient {
    client: reqwest::Client,
    base_url: String,
}

impl WorldBankClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.worldbank.org/v2".to_string(),
        }
    }

    pub async fn fetch_latest_indicator(
        &self,
        country_code: &str,
        indicator: &str,
    ) -> Result<Option<WorldBankDataPoint>, PopulationServiceError> {
        let url = format!(
            "{}/country/{}/indicator/{}",
            self.base_url, country_code, indicator
        );

        let response = self
            .client
            .get(&url)
            .query(&[("format", "json"), ("MRV", "1")])
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(PopulationServiceError::UnexpectedStatus(response.status()));
        }

        let json: serde_json::Value = response.json().await?;

        let point = json
            .get(1)
            .and_then(|data| data.get(0))
            .cloned()
            .map(serde_json::from_value)
            .transpose()?;

        Ok(point)
    }

    pub async fn fetch_latest_population_points(
        &self,
    ) -> Result<Vec<WorldBankDataPoint>, PopulationServiceError> {
        let indicator_codes = POPULATION_INDICATORS
            .iter()
            .map(|(_, code)| *code)
            .collect::<Vec<_>>()
            .join(";");
        let url = format!(
            "{}/country/all/indicator/{}",
            self.base_url, indicator_codes
        );

        let response = self
            .client
            .get(&url)
            .query(&[
                ("format", "json"),
                ("MRV", "1"),
                ("source", "2"),
                ("per_page", "20000"),
            ])
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PopulationServiceError::UnexpectedStatus(response.status()));
        }

        let json: serde_json::Value = response.json().await?;
        let points = json
            .get(1)
            .and_then(|data| data.as_array())
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .map(serde_json::from_value)
            .collect::<Result<Vec<WorldBankDataPoint>, serde_json::Error>>()?;

        Ok(points)
    }
}
