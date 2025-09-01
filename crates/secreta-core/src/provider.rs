use anyhow::Result;
use secrecy::SecretString;
use url::Url;

pub trait Provider: Sized + Send {
    fn new() -> impl Future<Output = Result<Self>> + Send;

    fn read(&self, resource: &Url) -> impl Future<Output = Result<SecretString>> + Send;
}
