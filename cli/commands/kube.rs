use crate::args::kube_args::{
    KubernetesArgs, 
    KubernetesCommand, 
    DeployArgs, 
    DeployCommand, 
    MachineTargets, 
    DeployMachineArgs
};
use crate::commands::CommandResult;
use crate::services::kube::local_cluster::{LocalCluster, LocalClusterCli};
use crate::services::cmd::ProcessCmd;
use crate::services::machine::deploy_local::{DeployMachine, DeployMachineCli};
use crate::services::kube::kubectl::KubectlCli;
use crate::services::kube::minikube::MinikubeCli;


pub fn run(args: KubernetesArgs) -> CommandResult {
    let local_cluster = LocalClusterCli::from_env()?;

    match args.command {
        KubernetesCommand::Mini => local_cluster.run(),
        KubernetesCommand::Deploy(args) => deploy(args),
    }

}

fn deploy(args: DeployArgs) -> CommandResult {
    match args.command {
        DeployCommand::Machine(args) => deploy_machine(args),
    }
}

const MACHINE_NAMESPACE: &str = "machine";

fn deploy_machine(args: DeployMachineArgs) -> CommandResult {
    let deploy_local = DeployMachineCli::new(
        MinikubeCli::new(ProcessCmd),
        KubectlCli::new(ProcessCmd),
    );

    match args.command {
        MachineTargets::Local => deploy_local.deploy(MACHINE_NAMESPACE),
    }
}