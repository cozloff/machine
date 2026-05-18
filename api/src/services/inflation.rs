use crate::models::inflation::{InflationRateRecord, InflationRateSnapshot};

mod error;
mod imf;

pub use error::InflationServiceError;
use imf::{IMF_INFLATION_INDICATORS, ImfClient};

pub struct InflationService {
    imf: ImfClient,
}

impl InflationService {
    pub fn new() -> Self {
        Self {
            imf: ImfClient::new(),
        }
    }

    pub async fn latest_for_country(
        &self,
        country_code: &str,
    ) -> Result<Option<InflationRateRecord>, InflationServiceError> {
        let point = self
            .imf
            .fetch_latest_indicator(country_code, "YOY_PCH_PA_PT")
            .await?;

        Ok(point.and_then(|point| {
            point.value.map(|value| InflationRateRecord {
                country_code: point.country_code,
                period: point.period,
                value,
            })
        }))
    }

    pub async fn snapshot_for_country(
        &self,
        country_code: &str,
    ) -> Result<Option<InflationRateSnapshot>, InflationServiceError> {
        let mut snapshot = None;

        for &(field, indicator) in IMF_INFLATION_INDICATORS {
            let Some(point) = self
                .imf
                .fetch_latest_indicator(country_code, indicator)
                .await?
            else {
                continue;
            };

            let snapshot = snapshot.get_or_insert_with(|| {
                InflationRateSnapshot::new(point.country_code.clone(), point.period.clone())
            });

            if field == "annual_percent_change" {
                snapshot.annual_percent_change = point.value;
            }
        }

        Ok(snapshot)
    }

    pub async fn snapshots_for_all_countries(
        &self,
    ) -> Result<Vec<InflationRateSnapshot>, InflationServiceError> {
        let points = self
            .imf
            .fetch_latest_indicator_for_all_countries("YOY_PCH_PA_PT")
            .await?;

        Ok(points
            .into_iter()
            .map(|point| InflationRateSnapshot {
                country_code: point.country_code,
                period: point.period,
                annual_percent_change: point.value,
            })
            .collect())
    }
}

impl Default for InflationService {
    fn default() -> Self {
        Self::new()
    }
}
