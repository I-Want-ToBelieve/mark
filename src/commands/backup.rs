use std::fs;
use std::path::Path;
use std::str;

use crate::config::Config;
use crate::schema::Toml;
use directories_next::{BaseDirs, UserDirs};
use ignore::Walk;
use rust_embed::RustEmbed;
use std::env::consts;
use toml;
extern crate glob;
use glob::glob;
use shadow_rs::shadow;
use spinach::Spinach;

shadow!(build);

#[derive(RustEmbed)]
#[folder = "src/apps/"]
struct Asset;

pub fn backup(cfg: &Config) -> Result<(), str::Utf8Error> {
    let user_dirs = UserDirs::new().unwrap();
    let base_dirs = BaseDirs::new().unwrap();

    let home_dir = user_dirs.home_dir().to_str().unwrap();
    let config_dir = base_dirs.config_dir().to_str().unwrap();

    let local_apps_path = Path::new(&config_dir)
        .join(build::PROJECT_NAME)
        .join("apps");

    let storage_dir = cfg.storage_path.replace("$HOME/", &format!("{home_dir}/"));

    println!("Backuping your app config to the {}.", &storage_dir);
    println!("Press Ctrl+C to cancel.");

    let storage_path = Path::new(&storage_dir).join(consts::OS);

    let parse = |content: &str| {
        let Toml { app } = toml::from_str(content).expect("toml file should spec");

        let paths = &app.path[consts::OS];

        paths.iter().for_each(|path| {
            let source = path.replace("$HOME/", &format!("{home_dir}/"));

            for entry in glob(&source).expect("Failed to read glob pattern") {
                match entry {
                    Ok(source) => {
                        let target = if source.starts_with(home_dir) {
                            storage_path.join(&source.to_str().unwrap().replace(home_dir, "HOME"))
                        } else {
                            storage_path.join(&source)
                        };

                        #[allow(clippy::single_match)]
                        match target.parent() {
                            Some(target_parent_dir) => {
                                let source = Path::new(&source);
                                if source.exists() {
                                    if !target_parent_dir.exists() {
                                        fs::create_dir_all(target_parent_dir)
                                            .expect("can't create dir all");
                                    }

                                    if source.is_dir() {
                                        for entry in Walk::new(&source) {
                                            let source = entry.expect("entry should be a dirEntry");
                                            let source = source.path();
                                            let target = if source.starts_with(home_dir) {
                                                storage_path.join(
                                                    &source
                                                        .to_str()
                                                        .unwrap()
                                                        .replace(home_dir, "HOME"),
                                                )
                                            } else {
                                                storage_path.join(&source)
                                            };

                                            if source.is_dir() {
                                                fs::create_dir_all(&target)
                                                    .expect("can't create dir all");
                                            } else {
                                                fs::copy(&source, &target)
                                                    .expect("can't copy file");
                                            }
                                        }
                                    } else if source.is_file() || source.is_symlink() {
                                        fs::copy(&source, &target).expect("can't copy file");
                                    } else {
                                        println!("WTF {}", source.display())
                                    }
                                }
                            }
                            None => {}
                        }
                    }
                    Err(e) => println!("{:?}", e),
                }
            }
        })
    };

    println!();

    let buildin_apps_spinach_text = format!("Copying buildin app config to {}", &storage_dir);
    let buildin_apps_spinach = Spinach::new(buildin_apps_spinach_text.clone());

    Asset::iter().for_each(|file_name| {
        let embedded_file = Asset::get(&file_name).unwrap();
        let content =
            str::from_utf8(embedded_file.data.as_ref()).expect("embedded_file is not utf8");

        parse(content);
    });

    buildin_apps_spinach.succeed(buildin_apps_spinach_text);

    let local_apps_spinach_text = format!("Copying local app config to {}", &storage_dir);
    let local_apps_spinach = Spinach::new(local_apps_spinach_text.clone());

    if local_apps_path.exists() {
        glob(local_apps_path.join("**/*.toml").to_str().unwrap())
            .expect("Failed to read glob pattern toml")
            .for_each(|path| {
                let entry = path.unwrap();
                let content = fs::read_to_string(entry).unwrap();

                parse(&content);
            })
    } else {
        fs::create_dir_all(local_apps_path).expect("should create local apps dir");
    };

    local_apps_spinach.succeed(local_apps_spinach_text);

    Ok(())
}
