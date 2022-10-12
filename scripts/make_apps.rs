//! Dependencies can be specified in the script file itself as follows:
//!
//! ```cargo
//! [dependencies]
//! cmd_lib = "1.3.0"
//! walkdir = "2"
//! configparser = "3.0.2"
//! toml = "0.5.9"
//! serde= { version = "1.0.145", features = ["derive"] }
//! ```

use cmd_lib::{run_cmd, CmdResult};
use configparser::ini::Ini;
use serde::Serialize;
use std::error::Error;
use std::fs;
use std::path::Path;
use toml;
use walkdir::WalkDir;

struct Options {
    repo: String,
    folder: String,
    dist: String,
}
fn download_git_repo_folder(opts: &Options) -> CmdResult {
    let Options { dist, repo, folder } = opts;
    run_cmd!(
        rm -rf $dist;
        svn export $repo/trunk/$folder $dist;
        rm -rf $dist/.git;
    )?;
    Ok(())
}

struct IniToTomlOptions {
    source: String,
    target: String,
}
fn ini_to_toml(
    IniToTomlOptions { source, target }: &IniToTomlOptions,
) -> Result<(), Box<dyn Error>> {
    WalkDir::new(&source)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            // if exists just skip
            !Path::new(&target)
                .join(Path::new(entry.file_name()).with_extension("toml"))
                .exists()
        })
        .for_each(|entry| {
            let mut config = Ini::new();
            if let Ok(map) = config.load(entry.path().to_str().unwrap()) {
                #[derive(Serialize)]
                struct Toml {
                    app: App,
                }

                #[derive(Serialize)]
                struct App {
                    name: String,
                    path: ConfigPath,
                }

                // https://doc.rust-lang.org/std/env/consts/constant.OS.html

                #[derive(Serialize)]
                struct ConfigPath {
                    linux: Vec<String>,
                    macos: Vec<String>,
                    windows: Vec<String>,
                }

                let xdg_configuration_files: Vec<String> = match map.get("xdg_configuration_files")
                {
                    None => [].to_vec(),
                    Some(xdg_configuration_files) => xdg_configuration_files
                        .keys()
                        .map(|key| format!("$HOME/.config/{key}"))
                        .collect(),
                };
                let configuration_files = match map.get("configuration_files") {
                    None => [].to_vec(),
                    Some(configuration_files) => configuration_files
                        .keys()
                        .map(|key| format!("$HOME/{key}"))
                        .collect(),
                };

                let all = [xdg_configuration_files, configuration_files].concat();
                let linux: Vec<String> = all
                    .clone()
                    .into_iter()
                    .filter(|key| !key.starts_with("$HOME/library"))
                    .collect();
                let macos = all;

                // println!("{:?}, {:?}", macos, linux);
                let toml_struct = Toml {
                    app: App {
                        name: config.get("application", "name").unwrap(),
                        path: ConfigPath {
                            linux,
                            macos,
                            windows: [].to_vec(),
                        },
                    },
                };
                let toml_string =
                    toml::to_string(&toml_struct).expect("Could not encode TOML value");
                fs::write(
                    Path::new(target).join(Path::new(entry.file_name()).with_extension("toml")),
                    toml_string,
                )
                .expect("Could not write to file!");
            }
        });
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = Options {
        repo: "https://github.com/lra/mackup".into(),
        folder: "mackup/applications".into(),
        dist: "./data/applications".into(),
    };
    match download_git_repo_folder(&opts) {
        Ok(()) => {
            let target = "./src/apps".into();
            match ini_to_toml(&IniToTomlOptions {
                source: opts.dist,
                target,
            }) {
                Ok(()) => {}
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    Ok(())
}
