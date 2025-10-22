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

#[derive(Debug)]
pub enum Mode {
    Real,
    Test,
}

impl Package {
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

