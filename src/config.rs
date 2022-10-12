use confy::{self, ConfyError};
use serde::{Deserialize, Serialize};
use shadow_rs::shadow;
use std::default::Default;

shadow!(build);

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub version: String,
    pub storage_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: build::PKG_VERSION.into(),
            storage_path: "$HOME/.dotfiles".into(),
        }
    }
}

pub fn get() -> Result<Config, ConfyError> {
    confy::load(build::PROJECT_NAME, build::PROJECT_NAME)
}
