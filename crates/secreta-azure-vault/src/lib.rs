use std::sync::Arc;

use anyhow::{Context, Result};
use azure_identity::DefaultAzureCredential;
use azure_security_keyvault_secrets::SecretClient;
use secrecy::SecretString;
use secreta_core::provider::Provider;
use url::Url;

pub struct AzureVaultProvider {
    credentials: Arc<DefaultAzureCredential>,
}

impl Provider for AzureVaultProvider {
    async fn new() -> Result<Self> {
        Ok(Self {
            credentials: DefaultAzureCredential::new()?,
        })
    }

    // azvault://<vault-name>/<secret-name>
    async fn read(&self, resource: &Url) -> Result<SecretString> {
        let vault = resource.host_str().context("Invalid resource")?;
        let client = SecretClient::new(
            &format!("https://{}.vault.azure.net/", vault),
            self.credentials.clone(),
            None,
        )?;

        let segments = resource
            .path_segments()
            .context("Missing secret name or key")?;

        let mut segments = segments;

        let secret_name = segments.next().context("Missing secret name")?;
        let secret_response = client.get_secret(secret_name, "", None).await?;
        let secret = secret_response.into_body().await?;

        Ok(SecretString::from(secret.value.unwrap_or_default()))
    }
}
