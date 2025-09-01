use anyhow::Result;
use k8s_openapi::api::core::v1::Secret;
use kube::Api;
use secrecy::SecretString;
use secreta_core::provider::Provider;
use url::Url;

pub struct KubernetesProvider {
    client: kube::Client,
}

impl Provider for KubernetesProvider {
    async fn new() -> Result<Self> {
        Ok(Self {
            client: kube::Client::try_default().await?,
        })
    }

    // kubernetes://<namespace>/<secret-name>/<secret-key>
    async fn read(&mut self, resource: &Url) -> Result<SecretString> {
        // let namespace = resource
        //     .host_str()
        //     .ok_or_else(|| anyhow::anyhow!("Invalid resource"))?;

        // TODO: This is not optimal
        let api: Api<Secret> =
            kube::Api::namespaced(self.client.clone(), resource.host_str().unwrap());

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

        let secret = api.get(secret_name).await?;
        let secret_content = secret
            .data
            .ok_or_else(|| anyhow::anyhow!("Secret data not found"))?;

        let secret_data = secret_content
            .get(secret_key)
            .ok_or_else(|| anyhow::anyhow!("Secret key not found"))?
            .to_owned();

        let secret_data = String::from_utf8_lossy(&secret_data.0);

        Ok(SecretString::from(secret_data.to_string()))
    }
}
