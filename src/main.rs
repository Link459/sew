use core::panic;
use serde::Deserialize;
use std::collections::HashMap;
use std::os::unix::fs;
use std::path::Path;
use std::process::abort;
use std::{env, fs::*};

#[derive(Deserialize, Debug, Default)]
struct Remap {
    from: String,
    to: String,
}

#[derive(Deserialize, Default, Debug)]
struct Config {
    src: String,
    dst: String,
    #[serde(default)]
    global_remap: Option<String>,
    #[serde(default)]
    remap: Vec<Remap>,
    #[serde(default)]
    ignore: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut start_dir = match env::args().skip(1).next() {
        Some(s) => s,
        None => ".".to_string(),
    };

    start_dir.push_str("/sew.toml");
    let mut config: Config;
    match read_to_string(start_dir).ok() {
        Some(config_file) => {
            config = toml::from_str(&config_file)?;
        }
        None => {
            panic!("No config file found. Create a sew.toml in the source directory");
        }
    }
    config.ignore.push("sew.toml".to_string());
    let config = dbg!(config);

    let remaps: HashMap<_, _> = config.remap.into_iter().map(|x| (x.from, x.to)).collect();

    let dir = read_dir(config.src)?;

    for file in dir {
        let file = file?;
        let mut link = config.dst.clone();
        let file_name = file.file_name().to_str().unwrap().to_string();
        if config.ignore.contains(&file_name) {
            continue;
        }

        if let Some(l) = remaps.get(&file_name) {
            link.push_str(l);
        } else {
            if let Some(ref remap) = config.global_remap {
                link.push_str(&remap);
            }
            link.push_str(&file_name);
        }
        let link = dbg!(link);
        dbg!(file.path());
    // ignoring errors to avoid it failing when the symlink already exists
        let _ = fs::symlink(file.path(), link);
    }
    return Ok(());
}
