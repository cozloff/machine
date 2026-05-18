use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct PopulationRecord {
    pub value: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct PopulationSnapshot {
    pub country_code: String,
    pub country_name: Option<String>,
    pub year: Option<String>,
    pub total: Option<f64>,
    pub growth_annual_percent: Option<f64>,
    pub density_per_sq_km: Option<f64>,
    pub urban_total: Option<f64>,
    pub urban_percent: Option<f64>,
    pub rural_total: Option<f64>,
    pub rural_percent: Option<f64>,
    pub female_total: Option<f64>,
    pub male_total: Option<f64>,
    pub age_0_to_14_total: Option<f64>,
    pub age_15_to_64_total: Option<f64>,
    pub age_65_plus_total: Option<f64>,
    pub birth_rate_per_1000: Option<f64>,
    pub death_rate_per_1000: Option<f64>,
    pub fertility_rate: Option<f64>,
    pub life_expectancy_years: Option<f64>,
}

impl PopulationSnapshot {
    pub fn new(country_code: String) -> Self {
        Self {
            country_code,
            country_name: None,
            year: None,
            total: None,
            growth_annual_percent: None,
            density_per_sq_km: None,
            urban_total: None,
            urban_percent: None,
            rural_total: None,
            rural_percent: None,
            female_total: None,
            male_total: None,
            age_0_to_14_total: None,
            age_15_to_64_total: None,
            age_65_plus_total: None,
            birth_rate_per_1000: None,
            death_rate_per_1000: None,
            fertility_rate: None,
            life_expectancy_years: None,
        }
    }

    pub fn set_indicator_value(&mut self, field: &str, value: f64) {
        match field {
            "total" => self.total = Some(value),
            "growth_annual_percent" => self.growth_annual_percent = Some(value),
            "density_per_sq_km" => self.density_per_sq_km = Some(value),
            "urban_total" => self.urban_total = Some(value),
            "urban_percent" => self.urban_percent = Some(value),
            "rural_total" => self.rural_total = Some(value),
            "rural_percent" => self.rural_percent = Some(value),
            "female_total" => self.female_total = Some(value),
            "male_total" => self.male_total = Some(value),
            "age_0_to_14_total" => self.age_0_to_14_total = Some(value),
            "age_15_to_64_total" => self.age_15_to_64_total = Some(value),
            "age_65_plus_total" => self.age_65_plus_total = Some(value),
            "birth_rate_per_1000" => self.birth_rate_per_1000 = Some(value),
            "death_rate_per_1000" => self.death_rate_per_1000 = Some(value),
            "fertility_rate" => self.fertility_rate = Some(value),
            "life_expectancy_years" => self.life_expectancy_years = Some(value),
            _ => {}
        }
    }
}
