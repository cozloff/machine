use crate::services::CommandResult;
use crate::services::kube::kubectl::Kubectl;
pub trait DeployMachine {
    fn deploy(&self, namespace: &str) -> CommandResult;
}

pub struct DeployMachineCli<K> {
    kubectl: K,
}

impl<K> DeployMachineCli<K> {
    pub fn new(kubectl: K) -> Self {
        Self { kubectl }
    }
}

impl<K> DeployMachine for DeployMachineCli<K>
where
    K: Kubectl,
{
    fn deploy(&self, namespace: &str) -> CommandResult {
        self.kubectl.create_namespace(namespace)?;
        self.kubectl.apply("k8s/ingest-api.yaml")?;

        Ok(())
    }
}
