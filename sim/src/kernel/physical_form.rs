use crate::kernel::boundary::QuoteRequest;
use crate::kernel::information::ShockState;
use crate::kernel::substrate::{MIN_MACHINE_TIME_MULTIPLIER, TRADING_DAYS_PER_YEAR, cost_if};

#[derive(Debug)]
pub struct ManufacturingConsequence {
    material_cost: f64,
    machine_cost: f64,
    labor_cost: f64,
    tooling_cost: f64,
    inspection_cost: f64,
    scrap_cost: f64,
    rework_cost: f64,
    deadline_penalty: f64,
}

impl ManufacturingConsequence {
    pub fn from_shocks(request: &QuoteRequest, shocks: &ShockState) -> Self {
        let material_cost = material_cost(request, shocks.material_return);
        let machine_hours = machine_hours(request, shocks.machine_time_return);
        let machine_cost = (request.setup_hours + machine_hours) * request.machine_hour_rate;
        let labor_cost = (request.setup_hours + request.labor_hours) * request.labor_hour_rate;

        let scrap_cost = if shocks.scrap {
            material_cost + (machine_cost * 0.50) + (request.tooling_cost * 0.25)
        } else {
            0.0
        };

        Self {
            material_cost,
            machine_cost,
            labor_cost,
            tooling_cost: request.tooling_cost,
            inspection_cost: request.inspection_cost,
            scrap_cost,
            rework_cost: cost_if(shocks.rework, request.rework_cost),
            deadline_penalty: cost_if(shocks.deadline_penalty, request.deadline_penalty_cost),
        }
    }

    pub fn total_cost_before_financing(&self) -> f64 {
        self.material_cost
            + self.machine_cost
            + self.labor_cost
            + self.tooling_cost
            + self.inspection_cost
            + self.scrap_cost
            + self.rework_cost
            + self.deadline_penalty
    }
}

fn material_cost(request: &QuoteRequest, random_return: f64) -> f64 {
    let years = request.days_until_paid as f64 / TRADING_DAYS_PER_YEAR;
    let variance_adjusted_drift =
        (request.annual_material_drift - 0.5 * request.annual_material_volatility.powi(2)) * years;
    let random_shock = request.annual_material_volatility * years.sqrt() * random_return;
    let material_price_multiplier = (variance_adjusted_drift + random_shock).exp();

    request.units * request.base_material_cost_per_unit * material_price_multiplier
}

fn machine_hours(request: &QuoteRequest, random_return: f64) -> f64 {
    let multiplier = 1.0 + (request.machine_time_std_dev_pct * random_return);
    request.base_machine_hours * multiplier.max(MIN_MACHINE_TIME_MULTIPLIER)
}
