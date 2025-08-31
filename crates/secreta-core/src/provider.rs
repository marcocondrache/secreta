use anyhow::Result;
use secrecy::SecretString;
use url::Url;

pub trait Provider: Sized + Send + Sync {
    fn new() -> impl Future<Output = Result<Self>> + Send + Sync;

    fn read(&mut self, resource: &Url) -> impl Future<Output = Result<SecretString>> + Send + Sync;
}
