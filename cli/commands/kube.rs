use crate::args::{KubernetesArgs, KubernetesCommand};
use crate::commands::CommandResult;
use crate::services::kube::local_cluster::{LocalCluster, LocalClusterCli};

pub fn run(args: KubernetesArgs) -> CommandResult {
    let local_cluster = LocalClusterCli::from_env()?;

    match args.command {
        KubernetesCommand::Mini => local_cluster.run(),
    }
}
