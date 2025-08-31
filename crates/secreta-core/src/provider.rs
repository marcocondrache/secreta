use secrecy::SecretString;
use anyhow::Result;
use url::Url;

pub trait Provider: Sized + Send + Sync {
    fn new() -> impl Future<Output = Result<Self>>;

    fn read(&mut self, resource: &Url) -> impl Future<Output = Result<SecretString>>;
}