use crate::services::population::{
    PopulationServiceError,
    ports::{POPULATION_INDICATORS, PopulationCountry, PopulationDataGateway, PopulationDataPoint},
};

mod dto;

use dto::WorldBankDataPoint;

pub struct WorldBankGateway {
    client: reqwest::Client,
    base_url: String,
}

impl WorldBankGateway {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.worldbank.org/v2".to_string(),
        }
    }
}

impl Default for WorldBankGateway {
    fn default() -> Self {
        Self::new()
    }
}

impl PopulationDataGateway for WorldBankGateway {
    async fn fetch_latest_indicator(
        &self,
        country_code: &str,
        indicator: &str,
    ) -> Result<Option<PopulationDataPoint>, PopulationServiceError> {
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
            .map(serde_json::from_value::<WorldBankDataPoint>)
            .transpose()?
            .map(PopulationDataPoint::from);

        Ok(point)
    }

    async fn fetch_latest_population_points(
        &self,
    ) -> Result<Vec<PopulationDataPoint>, PopulationServiceError> {
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
            .map(serde_json::from_value::<WorldBankDataPoint>)
            .map(|result| result.map(PopulationDataPoint::from))
            .collect::<Result<Vec<_>, serde_json::Error>>()?;

        Ok(points)
    }
}

impl From<WorldBankDataPoint> for PopulationDataPoint {
    fn from(point: WorldBankDataPoint) -> Self {
        Self {
            country: PopulationCountry {
                id: point.country.id,
                value: point.country.value,
            },
            country_iso3_code: point.countryiso3code,
            date: point.date,
            indicator_code: point.indicator.and_then(|indicator| indicator.id),
            value: point.value,
        }
    }
}
