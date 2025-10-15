use serde::Deserialize;
use url::Url;

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub url: Url,
    pub mode: String,
    pub output: String,
    pub depth: i32,
}
