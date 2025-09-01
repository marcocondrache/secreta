use anyhow::Result;
use secrecy::SecretString;
use secreta_core::provider::Provider;
use secreta_kubernetes::KubernetesProvider;
use url::Url;

// TODO: Avoid creating a new provider for each request
pub async fn route(schema: &str) -> Result<impl Provider> {
    match schema {
        "kubernetes" => KubernetesProvider::new().await,
        _ => Err(anyhow::anyhow!(
            "Invalid schema: {}, couldn't find a provider",
            schema
        )),
    }
}

pub async fn extract<P>(provider: &P, resource: &Url) -> Result<SecretString>
where
    P: Provider,
{
    provider.read(resource).await
}
