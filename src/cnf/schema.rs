use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_environment: Option<String>,
    pub environments: Vec<Environment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Environment {
    pub name: String,
    #[serde(default)]
    pub default: bool,
    // TODO: Uncomment this once we have a way to handle inheritance
    // pub extends: Option<String>,
    pub secrets: HashMap<String, Secret>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Secret {
    pub url: String,
    #[serde(default)]
    pub optional: bool,
    // TODO: Uncomment this once we have a way to handle matching
    // pub matcher: Option<String>,
}

impl Config {
    pub fn get_default_environment(&self) -> Option<&Environment> {
        if let Some(ref name) = self.default_environment {
            return self.get_environment(name);
        }

        self.environments.iter().find(|env| env.default)
    }

    pub fn get_environment(&self, name: &str) -> Option<&Environment> {
        self.environments.iter().find(|env| env.name == name)
    }
}

impl Environment {
    pub fn get_secret(&self, name: &str) -> Option<&Secret> {
        self.secrets.get(name)
    }
}
