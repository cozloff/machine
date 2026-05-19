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
    #[serde(default = "default_simulation_seed")]
    pub simulation_seed: u64,
    #[serde(default)]
    pub payment: Option<PaymentInstructions>,
    #[serde(default)]
    pub outreach: Option<OutreachInstructions>,
    #[serde(default)]
    pub reality_observation: Option<RealityObservationInput>,
}

#[derive(Debug, Deserialize)]
pub struct PaymentInstructions {
    #[serde(default = "default_payment_provider")]
    pub provider: String,
    pub recipient_name: String,
    pub payment_url: String,
    #[serde(default)]
    pub memo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct OutreachInstructions {
    #[serde(default = "default_outreach_provider")]
    pub provider: String,
    pub to: String,
    pub from: String,
    #[serde(default)]
    pub subject: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RealityObservationInput {
    pub customer_response: String,
    pub actual_total_cost: f64,
    pub actual_cash_received: f64,
    #[serde(default)]
    pub actual_cash_shortfall: Option<f64>,
    #[serde(default)]
    pub scrap: Option<bool>,
    #[serde(default)]
    pub rework: Option<bool>,
    #[serde(default)]
    pub late: Option<bool>,
}

fn default_simulation_seed() -> u64 {
    1
}

fn default_payment_provider() -> String {
    String::from("payment_link")
}

fn default_outreach_provider() -> String {
    String::from("email")
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

        if let Some(payment) = &self.payment {
            assert!(
                !payment.provider.trim().is_empty(),
                "payment.provider must not be empty"
            );
            assert!(
                !payment.recipient_name.trim().is_empty(),
                "payment.recipient_name must not be empty"
            );
            assert!(
                !payment.payment_url.trim().is_empty(),
                "payment.payment_url must not be empty"
            );
        }

        if let Some(outreach) = &self.outreach {
            assert!(
                !outreach.provider.trim().is_empty(),
                "outreach.provider must not be empty"
            );
            assert!(
                !outreach.to.trim().is_empty(),
                "outreach.to must not be empty"
            );
            assert!(
                !outreach.from.trim().is_empty(),
                "outreach.from must not be empty"
            );
        }

        if let Some(observation) = &self.reality_observation {
            assert!(
                observation.actual_total_cost >= 0.0,
                "reality_observation.actual_total_cost must be non-negative"
            );
            assert!(
                observation.actual_cash_received >= 0.0,
                "reality_observation.actual_cash_received must be non-negative"
            );
            if let Some(shortfall) = observation.actual_cash_shortfall {
                assert!(
                    shortfall >= 0.0,
                    "reality_observation.actual_cash_shortfall must be non-negative"
                );
            }
        }

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
