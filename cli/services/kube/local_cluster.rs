use crate::services::CommandResult;
use crate::services::cmd::{Cmd, ProcessCmd};
use crate::services::kube::cert_manager::{CertManager, CertManagerCli};
use crate::services::kube::gateway_api::{GatewayApi, GatewayApiCli};
use crate::services::kube::helm::HelmCli;
use crate::services::kube::kubectl::{Kubectl, KubectlCli};
use crate::services::kube::observability::alloy::AlloyCli;
use crate::services::kube::observability::loki_grafana::LokiGrafanaCli;
use crate::services::kube::observability::prometheus::PrometheusCli;
use crate::services::kube::observability::{
    NAMESPACE as OBSERVABILITY_NAMESPACE, Observability, ObservabilityCli,
    require_observability_env,
};
use anyhow::{Context, anyhow};
use std::{thread, time::Duration};

const GATEWAY_NAMESPACE: &str = "nginx-gateway";

pub trait LocalCluster {
    fn run(&self) -> CommandResult;
}

pub trait ApplicationInfra {
    fn deploy_infrastructure(&self) -> CommandResult;
}

pub struct NoopApplicationInfra;

impl ApplicationInfra for NoopApplicationInfra {
    fn deploy_infrastructure(&self) -> CommandResult {
        println!("Application infrastructure deployment is not configured. Skipping.");
        Ok(())
    }
}

pub struct LocalClusterCli {
    cmd: Box<dyn Cmd>,
    kubectl: Box<dyn Kubectl>,
    gateway_api: Box<dyn GatewayApi>,
    cert_manager: Box<dyn CertManager>,
    observability: Box<dyn Observability>,
    application_infra: Box<dyn ApplicationInfra>,
}

impl LocalClusterCli {
    pub fn new(
        cmd: Box<dyn Cmd>,
        kubectl: Box<dyn Kubectl>,
        gateway_api: Box<dyn GatewayApi>,
        cert_manager: Box<dyn CertManager>,
        observability: Box<dyn Observability>,
        application_infra: Box<dyn ApplicationInfra>,
    ) -> Self {
        Self {
            cmd,
            kubectl,
            gateway_api,
            cert_manager,
            observability,
            application_infra,
        }
    }

    pub fn from_env() -> CommandResult<Self> {
        let observability_env = require_observability_env()?;
        let grafana_pw = observability_env
            .get("GRAFANA_ADMIN_PASSWORD")
            .cloned()
            .expect("GRAFANA_ADMIN_PASSWORD should be loaded by require_observability_env");

        let cert_manager = CertManagerCli::new(
            HelmCli::new(ProcessCmd),
            KubectlCli::new(ProcessCmd),
            ProcessCmd,
        );
        let gateway_api = GatewayApiCli::new(
            HelmCli::new(ProcessCmd),
            KubectlCli::new(ProcessCmd),
            ProcessCmd,
        );
        let observability = ObservabilityCli::new(
            KubectlCli::new(ProcessCmd),
            observability_env,
            LokiGrafanaCli::new(OBSERVABILITY_NAMESPACE, HelmCli::new(ProcessCmd)),
            PrometheusCli::new(
                OBSERVABILITY_NAMESPACE,
                grafana_pw,
                HelmCli::new(ProcessCmd),
            ),
            AlloyCli::new(OBSERVABILITY_NAMESPACE, HelmCli::new(ProcessCmd)),
        );

        Ok(Self::new(
            Box::new(ProcessCmd),
            Box::new(KubectlCli::new(ProcessCmd)),
            Box::new(gateway_api),
            Box::new(cert_manager),
            Box::new(observability),
            Box::new(NoopApplicationInfra),
        ))
    }

    fn pre_flight_checks(&self) -> CommandResult {
        for binary in ["kubectl", "helm"] {
            which::which(binary)
                .with_context(|| format!("required binary `{binary}` was not found in PATH"))?;
        }

        Ok(())
    }

    fn wait_for_cluster(&self) -> CommandResult {
        println!("Waiting for k3s Kubernetes API...");

        for _ in 0..60 {
            if self.cmd.ok("kubectl", &["get", "nodes"]) {
                return Ok(());
            }

            thread::sleep(Duration::from_secs(2));
        }

        Err(anyhow!(
            "k3s Kubernetes API was not ready after 120 seconds; check `docker compose logs k3s`"
        ))
    }
}

impl LocalCluster for LocalClusterCli {
    fn run(&self) -> CommandResult {
        self.pre_flight_checks()?;
        self.wait_for_cluster()?;

        println!("{}", self.gateway_api.get_info());

        self.gateway_api.install_nginx_crds()?;
        self.cert_manager.setup_cert_manager()?;
        self.kubectl.create_namespace(GATEWAY_NAMESPACE)?;
        self.cert_manager.setup_certificates_for_gateway()?;
        self.gateway_api.install_gateway()?;
        self.kubectl.create_namespace(OBSERVABILITY_NAMESPACE)?;
        self.observability.apply_secrets()?;
        self.observability.install_prometheus()?;
        self.observability.install_loki_grafana()?;
        self.observability.install_alloy()?;
        self.application_infra.deploy_infrastructure()
    }
}
