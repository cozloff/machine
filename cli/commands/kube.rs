use crate::args::kube_args::{
    DeployArgs, DeployCommand, DeployMachineArgs, ForwardArgs, ForwardCommand, KubernetesArgs,
    KubernetesCommand, MachineTargets, PortForwardArgs,
};
use crate::commands::CommandResult;
use crate::services::cmd::{Cmd, ProcessCmd};
use crate::services::kube::kubectl::KubectlCli;
use crate::services::kube::local_cluster::{LocalCluster, LocalClusterCli};
use crate::services::machine::deploy_local::{DeployMachine, DeployMachineCli};

pub fn run(args: KubernetesArgs) -> CommandResult {
    let local_cluster = LocalClusterCli::from_env()?;

    match args.command {
        KubernetesCommand::K3s => local_cluster.run(),
        KubernetesCommand::Deploy(args) => deploy(args),
        KubernetesCommand::Forward(args) => forward(args),
    }
}

fn deploy(args: DeployArgs) -> CommandResult {
    match args.command {
        DeployCommand::Machine(args) => deploy_machine(args),
    }
}

const MACHINE_NAMESPACE: &str = "machine";
const OBSERVABILITY_NAMESPACE: &str = "observability";

fn deploy_machine(args: DeployMachineArgs) -> CommandResult {
    let deploy_local = DeployMachineCli::new(KubectlCli::new(ProcessCmd));

    match args.command {
        MachineTargets::Local => deploy_local.deploy(MACHINE_NAMESPACE),
    }
}

fn forward(args: ForwardArgs) -> CommandResult {
    match args.command {
        ForwardCommand::Grafana(args) => forward_grafana(args),
    }
}

fn forward_grafana(args: PortForwardArgs) -> CommandResult {
    let local_port = args.local_port.to_string();
    let port_mapping = format!("{local_port}:80");

    println!("Forwarding Grafana at http://localhost:{local_port}");
    println!("Press Ctrl-C to stop.");

    ProcessCmd.run(
        "kubectl",
        &[
            "port-forward",
            "--address",
            "0.0.0.0",
            "-n",
            OBSERVABILITY_NAMESPACE,
            "svc/monitoring-grafana",
            &port_mapping,
        ],
    )
}
