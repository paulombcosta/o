use crate::cmd::Cmd;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub cmds: Vec<Cmd>,
}

pub fn get_config() -> Config {
    let mut config_dir = dirs::home_dir().unwrap();
    config_dir.push(PathBuf::from(".o/config.json"));
    print!("config file = {:?}", config_dir);
    let mut config_file = File::open(config_dir).unwrap();
    let mut data = String::new();
    config_file.read_to_string(&mut data).unwrap();
    let config = serde_json::from_str(&data).unwrap();
    return config;
}

pub fn get_config_folder() -> PathBuf {
    dirs::home_dir().unwrap().join(".o")
}
