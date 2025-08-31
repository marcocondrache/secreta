use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    enviroments: HashMap<String, Environment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Environment {
    secrets: Vec<Secret>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Secret {
    url: Url,
    name: String,
    matcher: Option<String>,
}
