use anyhow::Result;
use clap::Args;
use secrecy::ExposeSecret;

use crate::{cnf, pvd};

#[derive(Debug, Args)]
pub struct ReadCommandArguments {
    #[arg(short, long, help = "The environment to use")]
    environment: Option<String>,

    #[arg(short, long, help = "The secret to read")]
    secret: String,
}

pub async fn init(args: ReadCommandArguments) -> Result<()> {
    let config = cnf::extract().await?;

    let default_enviroment = config
        .enviroments
        .values()
        .find(|e| e.default)
        .ok_or_else(|| anyhow::anyhow!("Default environment not found"))?;

    let enviroment = match args.environment {
        Some(environment) => config
            .enviroments
            .get(environment.as_str())
            .ok_or_else(|| anyhow::anyhow!("Environment not found"))?,
        None => default_enviroment,
    };

    let secret = enviroment
        .secrets
        .iter()
        .find(|s| s.name == args.secret)
        .ok_or_else(|| anyhow::anyhow!("Secret not found"))?;

    let url = pvd::render(&secret.url, enviroment).await?;
    let mut provider = pvd::route(&url.scheme()).await?;
    let value = pvd::extract(&mut provider, &url).await?;

    println!("{}", value.expose_secret());

    Ok(())
}
