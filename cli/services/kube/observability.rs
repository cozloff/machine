use crate::services::CommandResult;
use crate::services::kube::kubectl::Kubectl;
use std::collections::HashMap;
use std::env;
use std::io;

pub mod alloy;
pub mod loki_grafana;
pub mod prometheus;

use alloy::Alloy;
use loki_grafana::LokiGrafana;
use prometheus::Prometheus;

pub const NAMESPACE: &str = "observability";

const REQUIRED_ENV: [&str; 5] = [
    "LOKI_AZURE_STORAGE_ACCOUNT",
    "LOKI_AZURE_STORAGE_KEY",
    "LOKI_AZURE_BLOB_CONTAINER",
    "GRAFANA_ADMIN_PASSWORD",
    "LOKI_SCOPE_ORG_ID",
];

pub trait Observability {
    fn install_loki_grafana(&self) -> CommandResult;
    fn install_prometheus(&self) -> CommandResult;
    fn install_alloy(&self) -> CommandResult;
    fn apply_secrets(&self) -> CommandResult;
}

pub struct ObservabilityCli<K, L, P, A> {
    kubectl: K,
    env: HashMap<String, String>,
    loki_grafana: L,
    prometheus: P,
    alloy: A,
}

impl<K, L, P, A> ObservabilityCli<K, L, P, A> {
    pub fn new(
        kubectl: K,
        env: HashMap<String, String>,
        loki_grafana: L,
        prometheus: P,
        alloy: A,
    ) -> Self {
        Self {
            kubectl,
            env,
            loki_grafana,
            prometheus,
            alloy,
        }
    }
}

impl<K, L, P, A> Observability for ObservabilityCli<K, L, P, A>
where
    K: Kubectl,
    L: LokiGrafana,
    P: Prometheus,
    A: Alloy,
{
    fn install_loki_grafana(&self) -> CommandResult {
        self.loki_grafana.install()
    }

    fn install_prometheus(&self) -> CommandResult {
        self.prometheus.install()
    }

    fn install_alloy(&self) -> CommandResult {
        self.alloy.install()
    }

    fn apply_secrets(&self) -> CommandResult {
        self.kubectl.apply_generic_secret(
            "loki-azure-secret",
            NAMESPACE,
            &HashMap::from([
                (
                    "AZURE_STORAGE_ACCOUNT".to_owned(),
                    self.env_value("LOKI_AZURE_STORAGE_ACCOUNT")?.to_owned(),
                ),
                (
                    "AZURE_STORAGE_KEY".to_owned(),
                    self.env_value("LOKI_AZURE_STORAGE_KEY")?.to_owned(),
                ),
                (
                    "BLOB_NAME".to_owned(),
                    self.env_value("LOKI_AZURE_BLOB_CONTAINER")?.to_owned(),
                ),
            ]),
        )?;

        self.kubectl.apply_generic_secret(
            "grafana-loki-org",
            NAMESPACE,
            &HashMap::from([(
                "LOKI_SCOPE_ORG_ID".to_owned(),
                self.env_value("LOKI_SCOPE_ORG_ID")?.to_owned(),
            )]),
        )
    }
}

impl<K, L, P, A> ObservabilityCli<K, L, P, A> {
    fn env_value(&self, key: &str) -> CommandResult<&str> {
        self.env.get(key).map(String::as_str).ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("required environment variable {key} is missing"),
            )
            .into()
        })
    }
}

pub fn require_observability_env() -> CommandResult<HashMap<String, String>> {
    require_env(REQUIRED_ENV)
}

fn require_env<const N: usize>(keys: [&str; N]) -> CommandResult<HashMap<String, String>> {
    let mut values = HashMap::new();

    for key in keys {
        let value = env::var(key).map_err(|_| {
            io::Error::new(
                io::ErrorKind::NotFound,
                format!("required environment variable {key} is missing"),
            )
        })?;
        values.insert(key.to_owned(), value);
    }

    Ok(values)
}
