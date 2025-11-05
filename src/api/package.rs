use log::info;
use reqwest::blocking::get;
use ptree::{TreeItem, Style};
use serde::Deserialize;
use serde_json::from_str;
use core::str;
use std::{collections::HashMap, fs, path::PathBuf};

use crate::types::package::{MetaPackage, JSPackage};

const DEFAULT_PACKAGES_DIR: &str = "./tests/";
const DEFAULT_PACKAGE_META: &str = "package.json";
const DEFAULT_NPM_URL: &str = "https://registry.npmjs.org";


impl MetaPackage {
    pub fn request_main_get_package(&self) -> Result<JSPackage, Box<dyn std::error::Error>> {
        let response = get(self.url.clone())?;
        let package_serealize: JSPackage = response.json()?;

        Ok(package_serealize)
    }

    fn load_from_path(path: &String) -> Result<JSPackage, Box<dyn std::error::Error>> {
        let mut path_buf = std::path::PathBuf::from(path);

        if !path.ends_with(DEFAULT_PACKAGE_META) {
            path_buf.push(DEFAULT_PACKAGE_META);
        }

        let file_content = std::fs::read_to_string(&path_buf)?;
        let package: JSPackage = serde_json::from_str(&file_content)?;
        Ok(package)
    }

    pub fn file_get_package(&self) -> Result<JSPackage, Box<dyn std::error::Error>> {
        Self::load_from_path(&self.url)
    }
}

impl JSPackage {
    pub fn request_get_package(&self) -> Result<JSPackage, Box<dyn std::error::Error>> {
        let response = get(
            format!("{}/{}/{}", DEFAULT_NPM_URL, self.name, self.version.replace("^", ""))
        )?;
        let package_serealize: JSPackage = response.json()?;

        Ok(package_serealize)
    }

    pub fn file_get_package(&self) -> Result<JSPackage, Box<dyn std::error::Error>> {
        MetaPackage::load_from_path(
            &format!("{}/{}", DEFAULT_PACKAGES_DIR, self.name)
        )
    }
}


impl TreeItem for JSPackage {
    type Child = JSPackage;

    fn write_self<W: std::io::Write>(&self, f: &mut W, _style: &Style) -> std::io::Result<()> {
        write!(f, "{}@{}", self.name, self.version)
    }

    #[warn(mismatched_lifetime_syntaxes)]
    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        std::borrow::Cow::Borrowed(&self.dependencies)
    }
}