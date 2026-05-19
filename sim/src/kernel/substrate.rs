pub const TRADING_DAYS_PER_YEAR: f64 = 252.0;
pub const CALENDAR_DAYS_PER_YEAR: f64 = 365.0;
pub const MIN_MACHINE_TIME_MULTIPLIER: f64 = 0.10;

pub fn cost_if(condition: bool, cost: f64) -> f64 {
    if condition { cost } else { 0.0 }
}
