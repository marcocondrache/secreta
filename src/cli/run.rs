use std::collections::HashMap;

use anyhow::{Context, Result};
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
    let default_environment = config
        .get_default_environment()
        .context("Default environment not found")?;

    let environment = match args.environment {
        Some(environment) => config
            .get_environment(environment.as_str())
            .context("Environment not found")?,
        None => default_environment,
    };

    let mut set = JoinSet::new();
    for (name, secret) in &environment.secrets {
        set.spawn(async move {
            let url = pvd::render(&secret.url, &environment.name.as_str()).await?;
            let mut provider = pvd::route(&url.scheme()).await?;
            let value = pvd::extract(&mut provider, &url).await?;

            Ok((name.clone(), value))
        });
    }

    let results = set.join_all().await;
    let envs = results
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
