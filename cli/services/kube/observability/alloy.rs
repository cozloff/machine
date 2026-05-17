use crate::services::CommandResult;
use crate::services::kube::helm::Helm;

const GRAFANA_REPO_NAME: &str = "grafana";
const GRAFANA_REPO_URL: &str = "https://grafana.github.io/helm-charts";
const CHART_NAME: &str = "alloy";
const VALUES_FILE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/services/kube/observability/alloy/alloy-values.yaml"
);

pub trait Alloy {
    fn install(&self) -> CommandResult;
}

pub struct AlloyCli<H> {
    namespace: String,
    helm: H,
}

impl<H> AlloyCli<H> {
    pub fn new(namespace: impl Into<String>, helm: H) -> Self {
        Self {
            namespace: namespace.into(),
            helm,
        }
    }
}

impl<H: Helm> Alloy for AlloyCli<H> {
    fn install(&self) -> CommandResult {
        println!("Installing Alloy...");

        if self.helm.release_exists(CHART_NAME, &self.namespace) {
            return Ok(());
        }

        self.helm.ensure_repo(GRAFANA_REPO_NAME, GRAFANA_REPO_URL)?;
        self.helm.install_or_upgrade(
            CHART_NAME,
            &format!("{GRAFANA_REPO_NAME}/{CHART_NAME}"),
            &self.namespace,
            None,
            Some(VALUES_FILE),
        )
    }
}
