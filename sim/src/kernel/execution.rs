use std::time::Duration;

#[derive(Debug, Default)]
pub struct StageTiming {
    pub config_load: Duration,
    pub setup: Duration,
    pub simulation: Duration,
    pub random_sampling: Duration,
    pub physical_cost_model: Duration,
    pub currency_risk: Duration,
    pub aggregation: Duration,
    pub report: Duration,
    pub total: Duration,
}

pub fn print_timing_report(timing: &StageTiming) {
    println!();
    println!("Timing");
    print_timing("config load", timing.config_load);
    print_timing("setup", timing.setup);
    print_timing("simulation", timing.simulation);
    print_timing("  random sampling", timing.random_sampling);
    print_timing("  physical cost model", timing.physical_cost_model);
    print_timing("  currency risk", timing.currency_risk);
    print_timing("  aggregation", timing.aggregation);
    print_timing(
        "  timing overhead / unmeasured",
        timing
            .simulation
            .saturating_sub(timing.random_sampling)
            .saturating_sub(timing.physical_cost_model)
            .saturating_sub(timing.currency_risk)
            .saturating_sub(timing.aggregation),
    );
    print_timing("report", timing.report);
    print_timing("total", timing.total);
}

fn print_timing(label: &str, elapsed: Duration) {
    println!("  {label}: {:.3} ms", elapsed.as_secs_f64() * 1_000.0);
}
