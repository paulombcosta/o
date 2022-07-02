use crate::cmd::Cmd;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cmds: Vec<Cmd>,
}

pub fn get_config(mut config_path: PathBuf) -> Config {
    config_path.push(PathBuf::from("config.json"));
    let mut config_file = File::open(config_path).unwrap();
    let mut data = String::new();
    config_file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap()
}

pub fn get_default_config() -> Config {
    let mut config_folder = dirs::home_dir().unwrap();
    config_folder.push(PathBuf::from(".o"));
    get_config(config_folder)
}

pub fn get_config_folder() -> PathBuf {
    dirs::home_dir().unwrap().join(".o")
}
