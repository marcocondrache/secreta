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

    async fn read(&mut self, resource: &Url) -> Result<SecretString> {
        Ok(SecretString::new("".to_string().into()))
    }
}
