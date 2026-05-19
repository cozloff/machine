use crate::kernel::boundary::QuoteRequest;
use crate::kernel::physical_form::ManufacturingConsequence;
use crate::kernel::substrate::CALENDAR_DAYS_PER_YEAR;

#[derive(Debug)]
pub struct CurrencyConsequence {
    pub total_cost: f64,
    pub profit: f64,
    pub cash_shortfall: f64,
}

impl CurrencyConsequence {
    pub fn from_manufacturing(
        request: &QuoteRequest,
        manufacturing: &ManufacturingConsequence,
    ) -> Self {
        let pre_financing_cost = manufacturing.total_cost_before_financing();
        let financing_cost = pre_financing_cost
            * request.financing_annual_rate
            * (request.days_until_paid as f64 / CALENDAR_DAYS_PER_YEAR);
        let total_cost = pre_financing_cost + financing_cost;
        let profit = request.quote_price - total_cost;
        let cash_shortfall = f64::max(0.0, total_cost - request.cash_on_hand);

        Self {
            total_cost,
            profit,
            cash_shortfall,
        }
    }
}
