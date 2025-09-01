use anyhow::{Context, Result};
use clap::Args;
use secrecy::ExposeSecret;

use crate::{cnf, pvd, sec};

#[derive(Debug, Args)]
pub struct ReadCommandArguments {
    #[arg(short, long, help = "The environment to use")]
    environment: Option<String>,

    #[arg(short, long, help = "The secret to read")]
    secret: String,
}

pub async fn init(args: ReadCommandArguments) -> Result<()> {
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

    let secret = environment
        .get_secret(args.secret.as_str())
        .context("Secret not found")?;

    let url = sec::render(&secret.url, &environment.name).await?;
    let mut provider = pvd::route(&url.scheme()).await?;
    let value = pvd::extract(&mut provider, &url).await?;

    println!("{}", value.expose_secret());

    Ok(())
}
