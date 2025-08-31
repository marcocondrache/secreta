use std::collections::HashMap;

use anyhow::Result;
use clap::Args;
use secrecy::{ExposeSecret, SecretString};
use tokio::{process::Command, task::JoinSet};
use tracing::debug;

use crate::{cnf, pvd};

#[derive(Debug, Args)]
pub struct RunCommandArguments {
    #[arg(short, long, help = "The environment to use")]
    environment: Option<String>,

    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

pub async fn init(mut args: RunCommandArguments) -> Result<()> {
    if args.command.is_empty() {
        return Err(anyhow::anyhow!("No command provided"));
    }

    let config = cnf::extract().await?;
    let default_enviroment = &config.enviroments.1;

    let enviroment = match args.environment {
        Some(environment) => config
            .enviroments
            .0
            .get(environment.as_str())
            .ok_or_else(|| anyhow::anyhow!("Environment not found"))?,
        None => default_enviroment,
    };

    let mut set = JoinSet::new();
    for secret in &enviroment.secrets {
        set.spawn(async move {
            let mut provider = pvd::route(&secret.url).await?;
            let url = pvd::render(&secret.url, enviroment).await?;
            let value = pvd::extract(&mut provider, &url).await?;

            Ok((secret.name.clone(), value))
        });
    }

    let envs = set
        .join_all()
        .await
        .into_iter()
        .filter_map(|result: Result<(String, SecretString)>| result.ok())
        .map(|(name, value)| (name, value.expose_secret().to_string()))
        .collect::<HashMap<String, String>>();

    let status = Command::new(args.command.remove(0))
        .args(args.command)
        .envs(envs)
        .kill_on_drop(true)
        .status()
        .await?;

    debug!("Finished with status: {}", status);

    Ok(())
}
