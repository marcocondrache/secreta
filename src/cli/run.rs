use std::collections::HashMap;

use anyhow::Result;
use clap::Args;
use secrecy::ExposeSecret;
use tokio::process::Command;

use crate::{cnf, pvd};

#[derive(Debug, Args)]
pub struct RunCommandArguments {
    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

pub async fn init(mut args: RunCommandArguments) -> Result<()> {
    if args.command.is_empty() {
        return Err(anyhow::anyhow!("No command provided"));
    }

    let config = cnf::extract().await?;
    let secrets = config
        .enviroments
        .values()
        .find(|e| e.default)
        .map(|e| e.secrets.clone())
        .unwrap_or_else(|| vec![]);

    let mut envs: HashMap<String, String> = HashMap::new();
    for secret in secrets {
        let mut provider = pvd::route(&secret.url)?;
        let url = pvd::render(
            &secret.url,
            &config.enviroments.values().find(|e| e.default).unwrap(),
        )
        .await?;

        let value = pvd::extract(&mut provider, &url).await?;

        envs.insert(secret.name, value.expose_secret().to_string());
    }

    let _status = Command::new(args.command.remove(0))
        .args(args.command)
        .envs(envs)
        .kill_on_drop(true)
        .status()
        .await?;

    Ok(())
}
