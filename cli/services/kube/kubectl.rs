use crate::services::cmd::Cmd;
use crate::services::{CommandOutput, CommandResult};
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::io::Write;
use tempfile::NamedTempFile;

pub trait Kubectl {
    fn create_namespace(&self, namespace: &str) -> CommandResult;
    fn apply(&self, path: &str) -> CommandResult;
    fn apply_kustomize(&self, path: &str) -> CommandResult;
    fn secrets_exist(
        &self,
        namespace: &str,
        required_secrets: &HashSet<String>,
    ) -> CommandOutput<bool>;
    fn namespace_exists(&self, namespace: &str) -> bool;
    fn apply_generic_secret(
        &self,
        name: &str,
        namespace: &str,
        literals: &HashMap<String, String>,
    ) -> CommandResult;
    #[allow(dead_code)]
    fn rollout_restart(&self, resource: &str, namespace: &str) -> CommandResult;
}

pub struct KubectlCli<C> {
    cmd: C,
}

impl<C> KubectlCli<C> {
    pub fn new(cmd: C) -> Self {
        Self { cmd }
    }
}

impl<C: Cmd> Kubectl for KubectlCli<C> {
    fn create_namespace(&self, namespace: &str) -> CommandResult {
        if self.namespace_exists(namespace) {
            println!("Namespace '{namespace}' already exists. Skipping creation.");
            return Ok(());
        }

        self.cmd.run("kubectl", &["create", "namespace", namespace])
    }

    fn apply(&self, path: &str) -> CommandResult {
        self.cmd.run("kubectl", &["apply", "-f", path])
    }

    fn apply_kustomize(&self, path: &str) -> CommandResult {
        self.cmd.run("kubectl", &["apply", "-k", path])
    }

    fn secrets_exist(
        &self,
        namespace: &str,
        required_secrets: &HashSet<String>,
    ) -> CommandOutput<bool> {
        let output = self.cmd.capture(
            "kubectl",
            &["-n", namespace, "get", "secrets", "-o", "json"],
        )?;
        let secrets: Value = serde_json::from_str(&output)?;
        let existing_names: HashSet<String> = secrets["items"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|item| item["metadata"]["name"].as_str())
            .map(ToOwned::to_owned)
            .collect();

        Ok(required_secrets.is_subset(&existing_names))
    }

    fn namespace_exists(&self, namespace: &str) -> bool {
        self.cmd.ok("kubectl", &["get", "namespace", namespace])
    }

    fn apply_generic_secret(
        &self,
        name: &str,
        namespace: &str,
        literals: &HashMap<String, String>,
    ) -> CommandResult {
        let mut args = vec![
            "create".to_owned(),
            "secret".to_owned(),
            "generic".to_owned(),
            name.to_owned(),
            "--namespace".to_owned(),
            namespace.to_owned(),
            "--dry-run=client".to_owned(),
            "-o".to_owned(),
            "yaml".to_owned(),
        ];

        for (key, value) in literals {
            args.push(format!("--from-literal={key}={value}"));
        }

        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        let output = self.cmd.capture("kubectl", &arg_refs)?;

        let mut secret_file = NamedTempFile::new()?;
        secret_file.write_all(output.as_bytes())?;
        secret_file.flush()?;

        let secret_path = secret_file.path().to_string_lossy();
        self.apply(secret_path.as_ref())?;
        println!("Secret '{name}' applied/updated. success");

        Ok(())
    }

    fn rollout_restart(&self, resource: &str, namespace: &str) -> CommandResult {
        self.cmd.run(
            "kubectl",
            &["rollout", "restart", resource, "-n", namespace],
        )
    }
}
