use serde::Deserialize;
use std::collections::BTreeMap;

pub(in crate::services::inflation) type ImfInflationIndicator = (&'static str, &'static str);

pub(in crate::services::inflation) const IMF_INFLATION_INDICATORS: &[ImfInflationIndicator] =
    &[("annual_percent_change", "YOY_PCH_PA_PT")];

#[derive(Debug)]
pub(in crate::services::inflation) struct ImfDataPoint {
    pub country_code: String,
    pub period: String,
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::services::inflation) struct ImfSdmxResponse {
    pub data: ImfData,
}

impl ImfSdmxResponse {
    pub fn into_latest_data_point(self, country_code: &str) -> Option<ImfDataPoint> {
        let periods = self.data.periods().cloned().unwrap_or_default();

        self.data
            .data_sets
            .into_iter()
            .flat_map(|data_set| data_set.series.into_values())
            .flat_map(|series| series.observations.into_iter())
            .filter_map(|(period_index, observation)| {
                observation.into_data_point(country_code, &periods, &period_index)
            })
            .max_by(|left, right| left.period.cmp(&right.period))
    }

    pub fn into_latest_data_points(self) -> Vec<ImfDataPoint> {
        let Some(structure) = self.data.structures.first() else {
            return Vec::new();
        };
        let countries = structure
            .dimensions
            .series
            .first()
            .map(|dimension| dimension.values.clone())
            .unwrap_or_default();
        let periods = self.data.periods().cloned().unwrap_or_default();

        self.data
            .data_sets
            .into_iter()
            .flat_map(|data_set| data_set.series.into_iter())
            .filter_map(|(series_key, series)| {
                let country_code = country_code_from_series_key(&series_key, &countries)?;
                series
                    .observations
                    .into_iter()
                    .filter_map(|(period_index, observation)| {
                        observation.into_data_point(&country_code, &periods, &period_index)
                    })
                    .max_by(|left, right| left.period.cmp(&right.period))
            })
            .filter(|point| point.value.is_some())
            .collect()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::services::inflation) struct ImfData {
    pub data_sets: Vec<ImfDataSet>,
    pub structures: Vec<ImfStructure>,
}

impl ImfData {
    fn periods(&self) -> Option<&Vec<ImfDimensionValue>> {
        self.structures
            .first()
            .and_then(|structure| structure.dimensions.observation.first())
            .map(|dimension| &dimension.values)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(in crate::services::inflation) struct ImfDataSet {
    #[serde(default)]
    pub series: BTreeMap<String, ImfSeries>,
}

#[derive(Debug, Deserialize)]
pub(in crate::services::inflation) struct ImfSeries {
    #[serde(default)]
    pub observations: BTreeMap<String, Vec<serde_json::Value>>,
}

#[derive(Debug, Deserialize)]
pub(in crate::services::inflation) struct ImfStructure {
    pub dimensions: ImfDimensions,
}

#[derive(Debug, Deserialize)]
pub(in crate::services::inflation) struct ImfDimensions {
    pub series: Vec<ImfDimension>,
    pub observation: Vec<ImfDimension>,
}

#[derive(Debug, Deserialize)]
pub(in crate::services::inflation) struct ImfDimension {
    #[serde(default)]
    pub values: Vec<ImfDimensionValue>,
}

#[derive(Clone, Debug, Deserialize)]
pub(in crate::services::inflation) struct ImfDimensionValue {
    pub id: Option<String>,
    pub value: Option<String>,
}

impl ImfDimensionValue {
    fn identifier(&self) -> Option<String> {
        self.value.clone().or_else(|| self.id.clone())
    }
}

trait ImfObservationExt {
    fn into_data_point(
        self,
        country_code: &str,
        periods: &[ImfDimensionValue],
        period_index: &str,
    ) -> Option<ImfDataPoint>;
}

impl ImfObservationExt for Vec<serde_json::Value> {
    fn into_data_point(
        self,
        country_code: &str,
        periods: &[ImfDimensionValue],
        period_index: &str,
    ) -> Option<ImfDataPoint> {
        let value = self.first().and_then(|value| {
            value
                .as_f64()
                .or_else(|| value.as_str().and_then(|value| value.parse::<f64>().ok()))
        });
        let period_index = period_index.parse::<usize>().ok()?;
        let period = periods
            .get(period_index)
            .and_then(ImfDimensionValue::identifier)?;

        Some(ImfDataPoint {
            country_code: country_code.to_uppercase(),
            period,
            value,
        })
    }
}

fn country_code_from_series_key(
    series_key: &str,
    countries: &[ImfDimensionValue],
) -> Option<String> {
    let country_index = series_key.split(':').next()?.parse::<usize>().ok()?;
    countries
        .get(country_index)
        .and_then(ImfDimensionValue::identifier)
}
