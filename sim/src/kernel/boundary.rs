use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QuoteRequest {
    pub job_name: String,
    pub material_name: String,
    pub quote_price: f64,
    pub units: f64,
    pub base_material_cost_per_unit: f64,
    pub annual_material_volatility: f64,
    pub annual_material_drift: f64,
    pub base_machine_hours: f64,
    pub machine_hour_rate: f64,
    pub machine_time_std_dev_pct: f64,
    pub setup_hours: f64,
    pub labor_hours: f64,
    pub labor_hour_rate: f64,
    pub tooling_cost: f64,
    pub inspection_cost: f64,
    pub scrap_probability: f64,
    pub rework_probability: f64,
    pub rework_cost: f64,
    pub deadline_penalty_probability: f64,
    pub deadline_penalty_cost: f64,
    pub financing_annual_rate: f64,
    pub cash_on_hand: f64,
    pub days_until_paid: usize,
    pub simulations: usize,
}

impl QuoteRequest {
    pub fn validate(&self) {
        assert!(self.quote_price > 0.0, "quote_price must be positive");
        assert!(self.units > 0.0, "units must be positive");
        assert!(
            self.base_material_cost_per_unit >= 0.0,
            "base_material_cost_per_unit must be non-negative"
        );
        assert!(
            self.base_machine_hours >= 0.0,
            "base_machine_hours must be non-negative"
        );
        assert!(self.simulations > 0, "simulations must be positive");

        for (name, probability) in [
            ("scrap_probability", self.scrap_probability),
            ("rework_probability", self.rework_probability),
            (
                "deadline_penalty_probability",
                self.deadline_penalty_probability,
            ),
        ] {
            assert!(
                (0.0..=1.0).contains(&probability),
                "{name} must be between 0.0 and 1.0"
            );
        }
    }
}
