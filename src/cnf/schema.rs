use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub enviroments: HashMap<String, Environment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environment {
    pub default: bool,
    pub secrets: Vec<Secret>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Secret {
    pub url: String,
    pub name: String,
    pub matcher: Option<String>,
}
