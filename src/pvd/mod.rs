use std::collections::HashMap;

use anyhow::Result;
use secrecy::SecretString;
use secreta_core::provider::Provider;
use secreta_kubernetes::KubernetesProvider;
use url::Url;

use crate::cnf::schema::Environment;

pub async fn route(schema: &str) -> Result<impl Provider> {
    match schema {
        "kubernetes" => KubernetesProvider::new().await,
        _ => Err(anyhow::anyhow!(
            "Invalid schema: {}, couldn't find a provider",
            schema
        )),
    }
}

pub async fn render(raw_resource: &str, _enviroment: &Environment) -> Result<Url> {
    let tmp: HashMap<String, String> = HashMap::new();

    let template = leon::Template::parse(raw_resource)?;
    let resource = template.render(&tmp)?;

    Ok(Url::parse(&resource)?)
}

pub async fn extract(provider: &mut impl Provider, resource: &Url) -> Result<SecretString> {
    provider.read(resource).await
}
