use base64::prelude::*;

use anyhow::Result;
use k8s_openapi::api::core::v1::Secret;
use secrecy::SecretString;
use secreta_core::provider::Provider;
use url::Url;

pub struct KubernetesProvider {
    api: kube::Api<Secret>,
}

impl Provider for KubernetesProvider {
    async fn new() -> Result<Self> {
        Ok(Self {
            api: kube::Api::all(kube::Client::try_default().await?),
        })
    }

    // kubernetes://<namespace>/<secret-name>/<secret-key>
    async fn read(&mut self, resource: &Url) -> Result<SecretString> {
        // let namespace = resource
        //     .host_str()
        //     .ok_or_else(|| anyhow::anyhow!("Invalid resource"))?;

        let secret_name = resource
            .path_segments()
            .ok_or_else(|| anyhow::anyhow!("Invalid resource"))?
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid resource"))?;

        let secret_key = resource
            .path_segments()
            .ok_or_else(|| anyhow::anyhow!("Invalid resource"))?
            .nth(1)
            .ok_or_else(|| anyhow::anyhow!("Invalid resource"))?;

        let secret = self.api.get(secret_name).await?;
        let secret_content = secret
            .data
            .ok_or_else(|| anyhow::anyhow!("Secret data not found"))?;

        let secret_data = secret_content
            .get(secret_key)
            .ok_or_else(|| anyhow::anyhow!("Secret key not found"))?
            .to_owned();

        let decoded = BASE64_STANDARD.decode(secret_data.0)?;
        let secret_data = std::str::from_utf8(decoded.as_slice())?;

        Ok(SecretString::from(secret_data))
    }
}
