use crate::services::CommandResult;
use crate::services::cmd::Cmd;
use crate::services::kube::helm::Helm;
use crate::services::kube::kubectl::Kubectl;

const CRD_VERSION: &str = "v2.4.0";
const NAMESPACE: &str = "nginx-gateway";
const NGINX_URL: &str = "https://docs.nginx.com/nginx-gateway-fabric/install/secure-certificates/";
const NGINX_CHART_NAME: &str = "ngf";
const NGINX_CHART_URL: &str = "oci://ghcr.io/nginx/charts/nginx-gateway-fabric";
const SENTINEL_CRD: &str = "gateways.gateway.networking.k8s.io";
const NGINX_DEPLOYMENT: &str = "ngf-nginx-gateway-fabric";

pub trait GatewayApi {
    fn get_info(&self) -> String;
    fn install_nginx_crds(&self) -> CommandResult;
    fn install_gateway(&self) -> CommandResult;
}

pub struct GatewayApiCli<H, K, C> {
    helm: H,
    kubectl: K,
    cmd: C,
}

impl<H, K, C> GatewayApiCli<H, K, C> {
    pub fn new(helm: H, kubectl: K, cmd: C) -> Self {
        Self { helm, kubectl, cmd }
    }
}

impl<H: Helm, K: Kubectl, C: Cmd> GatewayApiCli<H, K, C> {
    fn gateway_crds_installed(&self) -> bool {
        self.cmd.ok("kubectl", &["get", "crd", SENTINEL_CRD])
    }

    fn deployment_exists(&self) -> bool {
        self.cmd.ok(
            "kubectl",
            &["get", "deployment", NGINX_DEPLOYMENT, "-n", NAMESPACE],
        )
    }

    fn wait_for_gateway(&self) -> CommandResult {
        self.cmd.run(
            "kubectl",
            &[
                "wait",
                "--timeout=5m",
                "-n",
                NAMESPACE,
                &format!("deployment/{NGINX_DEPLOYMENT}"),
                "--for=condition=Available",
            ],
        )
    }

    fn nginx_crd_path() -> String {
        format!(
            "github.com/nginx/nginx-gateway-fabric/config/crd/gateway-api/standard?ref={CRD_VERSION}"
        )
    }
}

impl<H: Helm, K: Kubectl, C: Cmd> GatewayApi for GatewayApiCli<H, K, C> {
    fn get_info(&self) -> String {
        format!("NGINX Gateway Fabric documentation can be found at {NGINX_URL}")
    }

    fn install_nginx_crds(&self) -> CommandResult {
        if self.gateway_crds_installed() {
            println!("NGINX Gateway Fabric CRDs are already installed.");
            return Ok(());
        }

        self.kubectl.apply_kustomize(&Self::nginx_crd_path())
    }

    fn install_gateway(&self) -> CommandResult {
        if self.deployment_exists() {
            println!("NGINX Gateway Fabric deployment already exists. Skipping installation.");
            return Ok(());
        }

        self.helm
            .install_chart(NGINX_CHART_NAME, NGINX_CHART_URL, NAMESPACE)?;
        self.wait_for_gateway()
    }
}
