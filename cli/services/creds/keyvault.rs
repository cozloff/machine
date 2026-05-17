use crate::services::CommandResult;
use crate::services::cmd::Cmd;
use std::io;
use std::path::{Path, PathBuf};

const ENV_SECRET_NAME: &str = "env";

pub trait Creds {
    fn az_login(&self) -> CommandResult;
    fn prompt_env(&self) -> CommandResult<String>;
    fn get_kv_name(&self, env_name: &str) -> CommandResult<String>;
    fn get_env(&self) -> CommandResult;
}

pub struct CredsCli<C> {
    cmd: C,
}

impl<C> CredsCli<C> {
    pub fn new(cmd: C) -> Self {
        Self { cmd }
    }

    fn env_path() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")
    }
}

impl<C: Cmd> Creds for CredsCli<C> {
    fn az_login(&self) -> CommandResult {
        // Check if valid credentials already exist or login
        if self.cmd.ok("az", &["account", "show"]) {
            println!("Already logged in to Azure. Skipping az login.");
            return Ok(());
        }

        println!("Not logged in to Azure. Running az login.");
        self.cmd.run("az", &["login"])?;
        Ok(())
    }

    fn prompt_env(&self) -> CommandResult<String> {
        println!("Please enter the name of the credential environment to use (e.g. 'home'):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_owned())
    }

    fn get_kv_name(&self, env_name: &str) -> CommandResult<String> {
        // Look for the keyvault under the expected environment name
        match env_name {
            "home" => Ok("kvmachinearzzxo".to_owned()),
            "work" => Ok("kv-machine-work".to_owned()),
            _ => Err(anyhow::anyhow!(
                "Unknown credential environment: {}",
                env_name
            )),
        }
    }

    fn get_env(&self) -> CommandResult {
        // Ensure we're logged in to Azure
        self.az_login()?;

        // Prompt for the credential environment to use
        let env_name: String = self.prompt_env()?;

        let kv_name: String = self.get_kv_name(&env_name)?;
        println!("Using Key Vault: {}", kv_name);

        let env_path = Self::env_path();
        let env_path = env_path.to_string_lossy();

        if !self.cmd.ok(
            "az",
            &[
                "keyvault",
                "secret",
                "show",
                "--vault-name",
                &kv_name,
                "--name",
                ENV_SECRET_NAME,
            ],
        ) {
            if !Path::new(env_path.as_ref()).is_file() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "Key Vault secret '{ENV_SECRET_NAME}' does not exist and local .env was not found at {}",
                        env_path
                    ),
                )
                .into());
            }

            println!(
                "Key Vault secret '{}' was not found. Uploading {} first.",
                ENV_SECRET_NAME, env_path
            );
            self.cmd.run(
                "az",
                &[
                    "keyvault",
                    "secret",
                    "set",
                    "--vault-name",
                    &kv_name,
                    "--name",
                    ENV_SECRET_NAME,
                    "--file",
                    env_path.as_ref(),
                ],
            )?;
        }

        println!(
            "Downloading Key Vault secret '{}' from '{}' to {}.",
            ENV_SECRET_NAME, kv_name, env_path
        );
        self.cmd.run(
            "az",
            &[
                "keyvault",
                "secret",
                "download",
                "--vault-name",
                &kv_name,
                "--name",
                ENV_SECRET_NAME,
                "--file",
                env_path.as_ref(),
            ],
        )?;

        Ok(())
    }
}
