use crate::{
    models::population::{PopulationRecord, PopulationSnapshot},
    repositories::population::PopulationRepository,
};

use std::collections::BTreeMap;

mod error;
mod world_bank;

pub use error::PopulationServiceError;
use world_bank::{POPULATION_INDICATORS, WorldBankClient};

pub struct PopulationService {
    repository: PopulationRepository,
    world_bank: WorldBankClient,
}

impl PopulationService {
    pub fn new() -> Self {
        Self {
            repository: PopulationRepository::sqlite(),
            world_bank: WorldBankClient::new(),
        }
    }

    pub async fn latest_for_country(
        &self,
        country_code: &str,
    ) -> Result<Option<PopulationRecord>, PopulationServiceError> {
        let point = self
            .world_bank
            .fetch_latest_indicator(country_code, "SP.POP.TOTL")
            .await?;

        Ok(point.and_then(|point| {
            point
                .value
                .and_then(|value| f64_to_u64(value).map(|value| PopulationRecord { value }))
        }))
    }

    pub async fn snapshot_for_country(
        &self,
        country_code: &str,
    ) -> Result<Option<PopulationSnapshot>, PopulationServiceError> {
        let mut snapshot = PopulationSnapshot::new(country_code.to_uppercase());
        let mut found_any_value = false;

        for &(field, code) in POPULATION_INDICATORS {
            let Some(point) = self
                .world_bank
                .fetch_latest_indicator(country_code, code)
                .await?
            else {
                continue;
            };

            if snapshot.country_name.is_none() {
                snapshot.country_name = Some(point.country.value);
            }

            if snapshot.year.is_none() {
                snapshot.year = Some(point.date);
            }

            if let Some(value) = point.value {
                found_any_value = true;
                snapshot.set_indicator_value(field, value);
            }
        }

        Ok(found_any_value.then_some(snapshot))
    }

    pub async fn snapshots_for_all_countries(
        &self,
    ) -> Result<Vec<PopulationSnapshot>, PopulationServiceError> {
        let points = self.world_bank.fetch_latest_population_points().await?;
        let indicator_fields = POPULATION_INDICATORS
            .iter()
            .map(|(field, code)| (*code, *field))
            .collect::<BTreeMap<_, _>>();
        let mut snapshots = BTreeMap::<String, (PopulationSnapshot, bool)>::new();

        for point in points {
            let Some(country_code) = point.countryiso3code.or(point.country.id) else {
                continue;
            };

            let entry = snapshots
                .entry(country_code.clone())
                .or_insert_with(|| (PopulationSnapshot::new(country_code), false));

            let snapshot = &mut entry.0;

            if snapshot.country_name.is_none() {
                snapshot.country_name = Some(point.country.value);
            }

            if snapshot.year.is_none() {
                snapshot.year = Some(point.date);
            }

            let Some(value) = point.value else {
                continue;
            };

            let Some(indicator_code) = point.indicator.and_then(|indicator| indicator.id) else {
                continue;
            };

            let Some(field) = indicator_fields.get(indicator_code.as_str()) else {
                continue;
            };

            entry.1 = true;
            snapshot.set_indicator_value(field, value);
        }

        Ok(snapshots
            .into_values()
            .filter_map(|(snapshot, found_any_value)| found_any_value.then_some(snapshot))
            .collect())
    }

    pub async fn save_all_countries_snapshot(&self) -> Result<(), PopulationServiceError> {
        let snapshots = self.snapshots_for_all_countries().await?;
        self.repository.save_snapshots(snapshots).await?;

        Ok(())
    }
}

impl Default for PopulationService {
    fn default() -> Self {
        Self::new()
    }
}

fn f64_to_u64(value: f64) -> Option<u64> {
    if value.is_finite() && value >= 0.0 && value.fract() == 0.0 {
        Some(value as u64)
    } else {
        None
    }
}
