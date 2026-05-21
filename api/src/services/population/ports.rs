use super::PopulationServiceError;

pub type PopulationIndicator = (&'static str, &'static str);

pub const POPULATION_INDICATORS: &[PopulationIndicator] = &[
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

#[derive(Debug)]
pub struct PopulationDataPoint {
    pub country: PopulationCountry,
    pub country_iso3_code: Option<String>,
    pub date: String,
    pub indicator_code: Option<String>,
    pub value: Option<f64>,
}

#[derive(Debug)]
pub struct PopulationCountry {
    pub id: Option<String>,
    pub value: String,
}

#[allow(async_fn_in_trait)]
pub trait PopulationDataGateway {
    async fn fetch_latest_indicator(
        &self,
        country_code: &str,
        indicator: &str,
    ) -> Result<Option<PopulationDataPoint>, PopulationServiceError>;

    async fn fetch_latest_population_points(
        &self,
    ) -> Result<Vec<PopulationDataPoint>, PopulationServiceError>;
}
