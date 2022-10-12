use serde::{Deserialize, Serialize};
use std::ops::Index;

#[derive(Serialize, Deserialize, Debug)]
pub struct Toml {
    pub app: App,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct App {
    pub name: String,
    pub path: ConfigPath,
}

// https://doc.rust-lang.org/std/env/consts/constant.OS.html

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigPath {
    pub linux: Vec<String>,
    pub macos: Vec<String>,
    pub windows: Vec<String>,
}

impl Index<&'_ str> for ConfigPath {
    type Output = Vec<String>;
    fn index(&self, os: &str) -> &Vec<std::string::String> {
        match os {
            "linux" => &self.linux,
            "macos" => &self.macos,
            "windows" => &self.windows,
            _ => panic!("unknown field: {}", os),
        }
    }
}
