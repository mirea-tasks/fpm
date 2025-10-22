use reqwest::blocking::get;
use serde::Deserialize;
use serde_json::from_str;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::types::package::Package;

const DEFAULT_PACKAGE_META: &str = "package.json";


#[derive(Debug, Deserialize)]
pub struct PackageJson {
    pub name: String,
    pub version: String,
    pub dependencies: Option<HashMap<String, String>>,
}

impl Package {
    pub fn request_main_get_package(&self) -> Result<PackageJson, Box<dyn std::error::Error>> {
        let response = get(self.url.clone())?;
        let package_serealize: PackageJson = response.json()?;

        Ok(package_serealize)
    }

    pub fn file_get_package(&self) -> Result<PackageJson, Box<dyn std::error::Error>> {
        let mut path_buf = if self.url.path().ends_with(DEFAULT_PACKAGE_META) {
            self.url
                .to_file_path()
                .map_err(|_| "incorrect file url, ex. file:///")?
        } else {
            let mut buf = self
                .url
                .to_file_path()
                .map_err(|_| "incorrect file url, ex. file:///")?;
            buf.push(DEFAULT_PACKAGE_META);
            buf
        };

        // TODO: its bad practice use file:/// for relative path
        path_buf = path_buf
            .to_str()
            .map(|s| s.trim_start_matches('/'))
            .map(PathBuf::from)
            .unwrap();

        let file_content = fs::read_to_string(&path_buf)?;
        let package: PackageJson = from_str(&file_content)?;

        Ok(package)
    }
}
