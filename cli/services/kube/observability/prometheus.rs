use crate::services::CommandResult;
use crate::services::kube::helm::Helm;
use std::collections::HashMap;

const PROMETHEUS_REPO_NAME: &str = "prometheus-community";
const PROMETHEUS_REPO_URL: &str = "https://prometheus-community.github.io/helm-charts";
const PROMETHEUS_RELEASE: &str = "monitoring";
const PROMETHEUS_CHART: &str = "kube-prometheus-stack";
const PROMETHEUS_VALUES_FILE: &str =
    "/workspace/cli/services/kube/observability/prometheus/values.local.yaml";

pub trait Prometheus {
    fn install(&self) -> CommandResult;
}

pub struct PrometheusCli<H> {
    namespace: String,
    grafana_pw: String,
    helm: H,
}

impl<H> PrometheusCli<H> {
    pub fn new(namespace: impl Into<String>, grafana_pw: impl Into<String>, helm: H) -> Self {
        Self {
            namespace: namespace.into(),
            grafana_pw: grafana_pw.into(),
            helm,
        }
    }
}

impl<H: Helm> Prometheus for PrometheusCli<H> {
    fn install(&self) -> CommandResult {
        println!("Installing Prometheus...");
        self.helm
            .ensure_repo(PROMETHEUS_REPO_NAME, PROMETHEUS_REPO_URL)?;

        println!("Installing or upgrading Prometheus stack...");
        let set_values =
            HashMap::from([("grafana.adminPassword".to_owned(), self.grafana_pw.clone())]);

        self.helm.install_or_upgrade(
            PROMETHEUS_RELEASE,
            &format!("{PROMETHEUS_REPO_NAME}/{PROMETHEUS_CHART}"),
            &self.namespace,
            Some(&set_values),
            Some(PROMETHEUS_VALUES_FILE),
        )
    }
}
