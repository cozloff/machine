use crate::services::CommandResult;
use crate::services::kube::helm::Helm;
use std::collections::HashMap;

const GRAFANA_REPO_NAME: &str = "grafana";
const GRAFANA_REPO_URL: &str = "https://grafana.github.io/helm-charts";
const CHART_NAME: &str = "loki";
const IMAGE_TAG: &str = "3.5.1";
const VALUES_FILE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/services/kube/observability/loki_grafana/values.operations.yaml"
);

pub trait LokiGrafana {
    fn install(&self) -> CommandResult;
}

pub struct LokiGrafanaCli<H> {
    namespace: String,
    helm: H,
}

impl<H> LokiGrafanaCli<H> {
    pub fn new(namespace: impl Into<String>, helm: H) -> Self {
        Self {
            namespace: namespace.into(),
            helm,
        }
    }
}

impl<H: Helm> LokiGrafana for LokiGrafanaCli<H> {
    fn install(&self) -> CommandResult {
        println!("Installing Loki and Grafana...");

        if self.helm.release_exists(CHART_NAME, &self.namespace) {
            return Ok(());
        }

        self.helm.ensure_repo(GRAFANA_REPO_NAME, GRAFANA_REPO_URL)?;

        let set_values = HashMap::from([("image.tag".to_owned(), IMAGE_TAG.to_owned())]);
        self.helm.install_or_upgrade(
            CHART_NAME,
            &format!("{GRAFANA_REPO_NAME}/{CHART_NAME}"),
            &self.namespace,
            Some(&set_values),
            Some(VALUES_FILE),
        )
    }
}
