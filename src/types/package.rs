use log::info;
use serde::de::Deserializer;
use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetaPackage {
    pub name: String,
    pub url: String,
    pub mode: String,
    pub output: String,
    pub depth: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JSPackage {
    pub name: String,
    pub version: String,

    #[serde(default)]
    pub url: String,

    #[serde(default, deserialize_with = "deserialize_dependencies")]
    pub dependencies: Vec<JSPackage>,
}

fn deserialize_dependencies<'de, D>(deserializer: D) -> Result<Vec<JSPackage>, D::Error>
where
    D: Deserializer<'de>,
{
    let dep_map: HashMap<String, String> = HashMap::deserialize(deserializer)?;

    let mut result = Vec::new();
    for (name, version_map) in dep_map {
        let package = JSPackage {
            name: name.clone(),
            url: String::new(),
            version: version_map.clone(),
            dependencies: Vec::new(),
        };
        result.push(package);
    }
    Ok(result)
}

#[derive(Debug, PartialEq)]
pub enum Mode {
    Real,
    Test,
}

impl MetaPackage {
    pub fn get_mode(&self) -> Result<Mode, String> {
        match self.mode.as_str() {
            "real" => Ok(Mode::Real),
            "test" => Ok(Mode::Test),
            other => Err(format!(
                "invalid mode '{}'. expected 'real' or 'test'.",
                other
            )),
        }
    }
}
