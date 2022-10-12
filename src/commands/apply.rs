use std::{env::consts, fs, path::Path};

use directories_next::UserDirs;
use rust_embed::RustEmbed;
use spinach::Spinach;

use crate::config::Config;
use walkdir::WalkDir;

#[derive(RustEmbed)]
#[folder = "src/apps/"]
struct Asset;

pub fn apply(cfg: &Config) -> Result<(), std::str::Utf8Error> {
    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir().to_str().expect("can't get home path");
        let storage_dir = cfg.storage_path.replace("$HOME/", &format!("{home_dir}/"));
        let storage_path = Path::new(&storage_dir).join(consts::OS);
        let storage_home = storage_path.join("HOME");

        let apps_spinach_text = format!("Applying app config from {}", &storage_dir);
        let apps_spinach = Spinach::new(apps_spinach_text.clone());

        WalkDir::new(&storage_path)
            .into_iter()
            .filter(|entry| {
                let entry = entry.as_ref().unwrap().path().to_path_buf();

                if entry == storage_path {
                    return false;
                }

                if entry == storage_home {
                    return false;
                }

                true
            })
            .for_each(|entry| {
                let source = entry.as_ref().unwrap().path();
                let target = source.to_str().unwrap().replace(
                    storage_home.to_str().unwrap(),
                    user_dirs.home_dir().to_str().unwrap(),
                );
                let target = Path::new(&target);

                if source.is_dir() {
                    fs::create_dir_all(&target).expect("can't create dir all");
                } else {
                    fs::copy(&source, &target).expect("can't copy file");
                }
            });

        apps_spinach.succeed(apps_spinach_text);
    }
    Ok(())
}
