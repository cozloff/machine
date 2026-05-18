use serde::Deserialize;

pub(in crate::services::population) type PopulationIndicator = (&'static str, &'static str);

pub(in crate::services::population) const POPULATION_INDICATORS: &[PopulationIndicator] = &[
    ("total", "SP.POP.TOTL"),
    ("growth_annual_percent", "SP.POP.GROW"),
    ("density_per_sq_km", "EN.POP.DNST"),
    ("urban_total", "SP.URB.TOTL"),
    ("urban_percent", "SP.URB.TOTL.IN.ZS"),
    ("rural_total", "SP.RUR.TOTL"),
    ("rural_percent", "SP.RUR.TOTL.ZS"),
    ("female_total", "SP.POP.TOTL.FE.IN"),
    ("male_total", "SP.POP.TOTL.MA.IN"),
    ("age_0_to_14_total", "SP.POP.0014.TO"),
    ("age_15_to_64_total", "SP.POP.1564.TO"),
    ("age_65_plus_total", "SP.POP.65UP.TO"),
    ("birth_rate_per_1000", "SP.DYN.CBRT.IN"),
    ("death_rate_per_1000", "SP.DYN.CDRT.IN"),
    ("fertility_rate", "SP.DYN.TFRT.IN"),
    ("life_expectancy_years", "SP.DYN.LE00.IN"),
];

#[derive(Debug, Deserialize)]
pub(in crate::services::population) struct WorldBankDataPoint {
    pub country: WorldBankCountry,
    pub countryiso3code: Option<String>,
    pub date: String,
    pub indicator: Option<WorldBankIndicator>,
    pub value: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub(in crate::services::population) struct WorldBankCountry {
    pub id: Option<String>,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub(in crate::services::population) struct WorldBankIndicator {
    pub id: Option<String>,
}
