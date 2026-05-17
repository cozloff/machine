use crate::services::CommandResult;
use crate::services::cmd::Cmd;
use serde_json::Value;
use std::collections::HashMap;

pub trait Helm {
    fn add_repo(&self, name: &str, url: &str) -> CommandResult;
    fn update_repos(&self) -> CommandResult;
    fn ensure_repo(&self, name: &str, url: &str) -> CommandResult;
    fn install_chart(&self, name: &str, chart: &str, namespace: &str) -> CommandResult;
    fn release_exists(&self, release: &str, namespace: &str) -> bool;
    #[allow(dead_code)]
    fn release_up_to_date(&self, release: &str, chart_name: &str) -> bool;
    fn install_or_upgrade(
        &self,
        release: &str,
        chart: &str,
        namespace: &str,
        set_values: Option<&HashMap<String, String>>,
        values_file: Option<&str>,
    ) -> CommandResult;
}

pub struct HelmCli<C> {
    cmd: C,
}

impl<C> HelmCli<C> {
    pub fn new(cmd: C) -> Self {
        Self { cmd }
    }
}

impl<C: Cmd> Helm for HelmCli<C> {
    fn add_repo(&self, name: &str, url: &str) -> CommandResult {
        self.cmd.run("helm", &["repo", "add", name, url])
    }

    fn update_repos(&self) -> CommandResult {
        self.cmd.run("helm", &["repo", "update"])
    }

    fn ensure_repo(&self, name: &str, url: &str) -> CommandResult {
        self.add_repo(name, url)?;
        self.update_repos()
    }

    fn install_chart(&self, name: &str, chart: &str, namespace: &str) -> CommandResult {
        self.cmd.run(
            "helm",
            &[
                "install",
                name,
                chart,
                "--create-namespace",
                "-n",
                namespace,
            ],
        )
    }

    fn release_exists(&self, release: &str, namespace: &str) -> bool {
        self.cmd.ok("helm", &["status", release, "-n", namespace])
    }

    fn release_up_to_date(&self, release: &str, chart_name: &str) -> bool {
        let Ok(output) = self.cmd.capture("helm", &["list", "-A", "-o", "json"]) else {
            return false;
        };

        if output.trim().is_empty() {
            return false;
        }

        let Ok(releases) = serde_json::from_str::<Vec<Value>>(&output) else {
            return false;
        };

        releases.iter().any(|item| {
            item["name"].as_str() == Some(release)
                && item["chart"]
                    .as_str()
                    .is_some_and(|chart| chart.starts_with(chart_name))
        })
    }

    fn install_or_upgrade(
        &self,
        release: &str,
        chart: &str,
        namespace: &str,
        set_values: Option<&HashMap<String, String>>,
        values_file: Option<&str>,
    ) -> CommandResult {
        let mut args = vec![
            "upgrade".to_owned(),
            "--install".to_owned(),
            release.to_owned(),
            chart.to_owned(),
            "-n".to_owned(),
            namespace.to_owned(),
            "--create-namespace".to_owned(),
        ];

        if let Some(values_file) = values_file {
            args.push("-f".to_owned());
            args.push(values_file.to_owned());
        }

        if let Some(set_values) = set_values {
            for (key, value) in set_values {
                args.push("--set-string".to_owned());
                args.push(format!("{key}={value}"));
            }
        }

        let arg_refs = args.iter().map(String::as_str).collect::<Vec<_>>();
        self.cmd.run("helm", &arg_refs)
    }
}
