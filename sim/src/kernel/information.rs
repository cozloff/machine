use crate::kernel::boundary::QuoteRequest;
use rand::Rng;
use rand_distr::{Distribution, StandardNormal};
use std::fs;

#[derive(Debug)]
pub struct ShockState {
    pub material_return: f64,
    pub machine_time_return: f64,
    pub scrap: bool,
    pub rework: bool,
    pub deadline_penalty: bool,
}

impl ShockState {
    pub fn sample<R: Rng + ?Sized>(request: &QuoteRequest, rng: &mut R) -> Self {
        Self {
            material_return: StandardNormal.sample(rng),
            machine_time_return: StandardNormal.sample(rng),
            scrap: rng.gen_bool(request.scrap_probability),
            rework: rng.gen_bool(request.rework_probability),
            deadline_penalty: rng.gen_bool(request.deadline_penalty_probability),
        }
    }
}

pub fn load_request(path: &str) -> QuoteRequest {
    let raw = fs::read_to_string(path).expect("failed to read config file");
    toml::from_str(&raw).expect("failed to parse config file")
}
