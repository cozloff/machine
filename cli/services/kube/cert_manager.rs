use crate::services::CommandResult;
use crate::services::cmd::Cmd;
use crate::services::kube::helm::Helm;
use crate::services::kube::kubectl::Kubectl;
use std::collections::HashSet;
use std::io;
use std::path::PathBuf;

const JETSTACK_REPO_NAME: &str = "jetstack";
const JETSTACK_REPO_URL: &str = "https://charts.jetstack.io";
const CERT_MANAGER_RELEASE: &str = "cert-manager";
const CERT_MANAGER_CHART: &str = "jetstack/cert-manager";
const CERT_MANAGER_NAMESPACE: &str = "cert-manager";
const NGINX_GATEWAY_NAMESPACE: &str = "nginx-gateway";
const CERT_MANAGER_MANIFEST_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/cli/services/kube/cert_manager"
);

pub trait CertManager {
    fn setup_cert_manager(&self) -> CommandResult;
    fn wait_for_certificate(&self, name: &str, namespace: &str, timeout: &str) -> CommandResult;
    fn setup_certificates_for_gateway(&self) -> CommandResult;
}

pub struct CertManagerCli<H, K, C> {
    helm: H,
    kubectl: K,
    cmd: C,
}

impl<H, K, C> CertManagerCli<H, K, C> {
    pub fn new(helm: H, kubectl: K, cmd: C) -> Self {
        Self { helm, kubectl, cmd }
    }
}

impl<H: Helm, K: Kubectl, C: Cmd> CertManagerCli<H, K, C> {
    fn rollout_status(&self, deployment: &str, namespace: &str, timeout: &str) -> CommandResult {
        self.cmd.run(
            "kubectl",
            &[
                "rollout",
                "status",
                deployment,
                "-n",
                namespace,
                &format!("--timeout={timeout}"),
            ],
        )
    }

    fn manifest_path(file_name: &str) -> String {
        PathBuf::from(CERT_MANAGER_MANIFEST_DIR)
            .join(file_name)
            .to_string_lossy()
            .into_owned()
    }

    fn required_secrets() -> HashSet<String> {
        ["agent-tls", "nginx-gateway-ca", "server-tls"]
            .into_iter()
            .map(ToOwned::to_owned)
            .collect()
    }
}

impl<H: Helm, K: Kubectl, C: Cmd> CertManager for CertManagerCli<H, K, C> {
    fn setup_cert_manager(&self) -> CommandResult {
        self.helm.add_repo(JETSTACK_REPO_NAME, JETSTACK_REPO_URL)?;
        self.helm.update_repos()?;

        self.cmd.run(
            "helm",
            &[
                "upgrade",
                "--install",
                CERT_MANAGER_RELEASE,
                CERT_MANAGER_CHART,
                "--namespace",
                CERT_MANAGER_NAMESPACE,
                "--create-namespace",
                "--set",
                "config.apiVersion=controller.config.cert-manager.io/v1alpha1",
                "--set",
                "config.kind=ControllerConfiguration",
                "--set",
                "config.enableGatewayAPI=true",
                "--set",
                "crds.enabled=true",
            ],
        )?;

        self.rollout_status("deployment/cert-manager", CERT_MANAGER_NAMESPACE, "120s")?;
        self.rollout_status(
            "deployment/cert-manager-webhook",
            CERT_MANAGER_NAMESPACE,
            "120s",
        )
    }

    fn wait_for_certificate(&self, name: &str, namespace: &str, timeout: &str) -> CommandResult {
        self.cmd.run(
            "kubectl",
            &[
                "-n",
                namespace,
                "wait",
                "--for=condition=Ready",
                &format!("certificate/{name}"),
                &format!("--timeout={timeout}"),
            ],
        )
    }

    fn setup_certificates_for_gateway(&self) -> CommandResult {
        let ca_path = Self::manifest_path("ca.yaml");
        let server_cert_path = Self::manifest_path("server_cert.yaml");
        let client_cert_path = Self::manifest_path("client_cert.yaml");

        self.kubectl.apply(&ca_path)?;
        self.kubectl.apply(&server_cert_path)?;
        self.kubectl.apply(&client_cert_path)?;

        self.wait_for_certificate("nginx-gateway", NGINX_GATEWAY_NAMESPACE, "60s")?;
        self.wait_for_certificate("nginx", NGINX_GATEWAY_NAMESPACE, "60s")?;

        if self
            .kubectl
            .secrets_exist(NGINX_GATEWAY_NAMESPACE, &Self::required_secrets())?
        {
            println!("All required TLS Secrets have been created.");
            return Ok(());
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "one or more required TLS Secrets are missing",
        )
        .into())
    }
}
