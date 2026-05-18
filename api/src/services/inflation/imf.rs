use super::InflationServiceError;

mod dto;

pub(in crate::services::inflation) use dto::IMF_INFLATION_INDICATORS;
use dto::{ImfDataPoint, ImfSdmxResponse};

const CPI_DATAFLOW: &str = "CPI_2026_APR_VINTAGE";

pub(super) struct ImfClient {
    client: reqwest::Client,
    base_url: String,
}

impl ImfClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: "https://api.imf.org/external/sdmx/3.0".to_string(),
        }
    }

    pub async fn fetch_latest_indicator(
        &self,
        country_code: &str,
        indicator: &str,
    ) -> Result<Option<ImfDataPoint>, InflationServiceError> {
        let series_key = format!("{}.CPI._T.{}.M", country_code.to_uppercase(), indicator);
        let url = format!(
            "{}/data/dataflow/IMF.STA/{CPI_DATAFLOW}/1.0.0/{series_key}",
            self.base_url
        );
        let response = self
            .client
            .get(&url)
            .query(&[
                ("lastNObservations", "1"),
                ("attributes", "dsd"),
                ("measures", "all"),
            ])
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.sdmx.data+json;version=2.0.0",
            )
            .send()
            .await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(InflationServiceError::UnexpectedStatus(response.status()));
        }

        let json: serde_json::Value = response.json().await?;
        let response: ImfSdmxResponse = serde_json::from_value(json)?;

        Ok(response.into_latest_data_point(country_code))
    }

    pub async fn fetch_latest_indicator_for_all_countries(
        &self,
        indicator: &str,
    ) -> Result<Vec<ImfDataPoint>, InflationServiceError> {
        let series_key = format!("*.CPI._T.{}.M", indicator);
        let url = format!(
            "{}/data/dataflow/IMF.STA/{CPI_DATAFLOW}/1.0.0/{series_key}",
            self.base_url
        );
        let response = self
            .client
            .get(&url)
            .query(&[
                ("lastNObservations", "1"),
                ("attributes", "dsd"),
                ("measures", "all"),
            ])
            .header(
                reqwest::header::ACCEPT,
                "application/vnd.sdmx.data+json;version=2.0.0",
            )
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(InflationServiceError::UnexpectedStatus(response.status()));
        }

        let json: serde_json::Value = response.json().await?;
        let response: ImfSdmxResponse = serde_json::from_value(json)?;

        Ok(response.into_latest_data_points())
    }
}
