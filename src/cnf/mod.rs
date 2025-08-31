use std::env;

use tokio::sync::OnceCell;
use anyhow::Result;

pub mod schema;

static CNF: OnceCell<schema::Config> = OnceCell::const_new();

pub async fn extract() -> Result<&'static schema::Config> {
    let path = match env::var("SECRETA_CONFIG") {
        Ok(path) => std::path::PathBuf::from(path),
        Err(_) => std::path::PathBuf::from(".secreta.yaml"),
    };

    let config = CNF
        .get_or_try_init(|| async {
            let config = tokio::fs::read(path).await?;
            let config: schema::Config = serde_yml::from_slice(&config)?;

            Ok::<_, anyhow::Error>(config)
        })
        .await?;

    Ok(config)
}