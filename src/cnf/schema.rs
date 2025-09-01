use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub enviroments: Enviroments,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(try_from = "HashMap<String, Environment>")]
pub struct Enviroments(pub HashMap<String, Environment>, pub Environment);

// Applies invariant that the default environment is present
// TODO: default can be optional, this is for simplicity now
impl TryFrom<HashMap<String, Environment>> for Enviroments {
    type Error = anyhow::Error;

    // TODO: Optimize this
    fn try_from(value: HashMap<String, Environment>) -> Result<Self, Self::Error> {
        let default = value
            .iter()
            .find(|(_, e)| e.default.clone())
            .ok_or_else(|| anyhow::anyhow!("Default environment not found"))?;

        Ok(Enviroments(value.clone(), default.1.clone()))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Environment {
    #[serde(default)]
    pub default: bool,
    pub secrets: Vec<Secret>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Secret {
    pub url: String,
    pub name: String,
    pub matcher: Option<String>,
}
