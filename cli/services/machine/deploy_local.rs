use crate::services::CommandResult;
use crate::services::kube::kubectl::Kubectl;
use crate::services::kube::minikube::Minikube;
use std::path::Path;

pub trait DeployMachine {
    fn deploy(&self, namespace: &str) -> CommandResult;
}

pub struct DeployMachineCli<M, K> {
    minikube: M,
    kubectl: K,
}

impl<M, K> DeployMachineCli<M, K> {
    pub fn new(minikube: M, kubectl: K) -> Self {
        Self { minikube, kubectl }
    }
}

impl<M, K> DeployMachine for DeployMachineCli<M, K>
where
    M: Minikube,
    K: Kubectl,
{
    fn deploy(&self, namespace: &str) -> CommandResult {
        self.minikube
            .image_build("ingest-api:local", Path::new("ingest/api"))?;

        self.kubectl.create_namespace(namespace)?;
        self.kubectl.apply("k8s/ingest-api.yaml")?;

        Ok(())
    }
}
