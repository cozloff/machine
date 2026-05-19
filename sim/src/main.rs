mod kernel;

use kernel::execution::{StageTiming, print_timing_report};
use kernel::information::load_request;
use kernel::policy::{RiskEngine, print_reality_signal, print_report};
use std::time::Instant;

fn main() {
    let total_started = Instant::now();
    let mut timing = StageTiming::default();

    let config_started = Instant::now();
    let request = load_request("config.toml");
    timing.config_load = config_started.elapsed();

    let setup_started = Instant::now();
    let engine = RiskEngine::new(request);
    timing.setup = setup_started.elapsed();

    let trace = engine.simulate(&mut timing);
    let decision = engine.decide(&trace);
    let reality_signal = engine.observe_reality(&decision, None);

    let report_started = Instant::now();
    print_report(engine.agent(), engine.request(), &decision);
    print_reality_signal(&reality_signal);
    timing.report = report_started.elapsed();

    timing.total = total_started.elapsed();
    print_timing_report(&timing);
}
