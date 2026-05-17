use crate::services::CommandResult;
use crate::services::cmd::Cmd;
use std::path::Path;

pub trait Minikube {
    fn boot(&self) -> CommandResult;
    #[allow(dead_code)]
    fn image_build(&self, tag: &str, dockerfile_dir: &Path) -> CommandResult;
}

pub struct MinikubeCli<C> {
    cmd: C,
}

impl<C> MinikubeCli<C> {
    pub fn new(cmd: C) -> Self {
        Self { cmd }
    }
}

struct MinikubeStatus {
    host: String,
    kubelet: String,
    apiserver: String,
}

impl MinikubeStatus {
    fn is_running(&self) -> bool {
        self.host == "Running" && self.kubelet == "Running" && self.apiserver == "Running"
    }
}

impl<C: Cmd> MinikubeCli<C> {
    fn get_status(&self) -> MinikubeStatus {
        let result = self.cmd.capture(
            "minikube",
            &["status", "--format={{.Host}}|{{.Kubelet}}|{{.APIServer}}"],
        );

        if let Ok(output) = result {
            let mut parts = output.trim().split('|');

            return MinikubeStatus {
                host: parts.next().unwrap_or("Unknown").to_owned(),
                kubelet: parts.next().unwrap_or("Unknown").to_owned(),
                apiserver: parts.next().unwrap_or("Unknown").to_owned(),
            };
        }

        MinikubeStatus {
            host: "Unknown".to_owned(),
            kubelet: "Unknown".to_owned(),
            apiserver: "Unknown".to_owned(),
        }
    }
}

impl<C: Cmd> Minikube for MinikubeCli<C> {
    fn boot(&self) -> CommandResult {
        let status = self.get_status();

        if status.is_running() {
            println!("Minikube is already running. Skipping boot.");
            return Ok(());
        }

        println!(
            "Minikube status: host={}, kubelet={}, apiserver={}",
            status.host, status.kubelet, status.apiserver
        );
        println!("Booting Minikube cluster...");

        self.cmd
            .run("minikube", &["start", "--driver=docker", "--wait=all"])
    }

    fn image_build(&self, tag: &str, dockerfile_dir: &Path) -> CommandResult {
        let dockerfile_dir = dockerfile_dir.to_string_lossy();

        self.cmd.run(
            "minikube",
            &["image", "build", "-t", tag, dockerfile_dir.as_ref()],
        )
    }
}
