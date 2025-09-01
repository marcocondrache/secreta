use anyhow::Result;
use secrecy::SecretString;
use url::Url;

use crate::{
    cnf::schema::{Environment, Secret},
    pvd,
};

pub fn render(raw_resource: &str, environment: &str) -> Result<Url> {
    let values = [("environment".to_string(), environment.to_string())];

    let template = leon::Template::parse(raw_resource)?;
    let resource = template.render(&values)?;

    Ok(Url::parse(&resource)?)
}

pub async fn fetch(secret: &Secret, environment: &Environment) -> Result<SecretString> {
    let url = render(&secret.url, &environment.name)?;
    let provider = pvd::route(url.scheme()).await?;
    let value = pvd::extract(&provider, &url).await?;

    Ok(value)
}
