use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(version)]
pub struct Args {
    /// Set new PS3 IP address to use
    #[arg(short, long)]
    pub new_ip: Option<Box<str>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserConfig {
    pub ps3_ip: Box<str>,
    pub sleep_duration: Duration,
    pub reconnect_delay: Duration,
}

impl UserConfig {
    pub fn new(ps3_ip: &str, sleep_duration: Duration, reconnect_delay: Duration) -> Self {
        return UserConfig {
            ps3_ip: ps3_ip.into(),
            sleep_duration,
            reconnect_delay,
        };
    }
}

#[derive(Debug)]
pub struct CurrentGame {
    pub id: String,
    pub name: String,
}

impl CurrentGame {
    pub fn new(id: &str, name: &str) -> Self {
        return CurrentGame {
            id: id.to_string(),
            name: name.to_string(),
        };
    }
}

pub fn get_config_path() -> Option<PathBuf> {
    let path = dirs::config_dir()?.join("ps3rpc").join("settings.toml");

    return Some(path);
}

pub fn get_cache_path() -> Option<PathBuf> {
    let path = dirs::cache_dir()?.join("ps3rpc");

    return Some(path);
}

pub fn create_config_dir(path: &PathBuf) -> Result<(), std::io::Error> {
    let p = path.parent().unwrap();
    fs::create_dir_all(p)?;

    Ok(())
}

pub fn create_empty_config(path: &PathBuf) -> Result<(), io::Error> {
    // Create the path first, then create the file
    create_config_dir(&path)?;
    fs::File::create(&path)?;

    Ok(())
}

pub fn load_user_config(path: &PathBuf) -> Result<UserConfig, ConfigReadError> {
    let cfg_string: String = fs::read_to_string(&path)?;

    let cfg = toml::from_str(&cfg_string)?;

    Ok(cfg)
}

pub fn save_user_config(cfg: &UserConfig, path: &PathBuf) {
    // TODO
}

pub fn flushed_print(str: &str) {
    print!("{}", str);
    std::io::stdout().flush().unwrap();
}

pub fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    return input;
}

pub fn any_key_continue() {
    flushed_print("Press enter to continue.");
    let _ = io::stdin().read_line(&mut String::new());
}

/* --- Custom Errors --- */
#[derive(Debug)]
pub enum ConfigWriteError {
    TomlSerError(toml::ser::Error),
    IoError(std::io::Error),
}

impl From<toml::ser::Error> for ConfigWriteError {
    fn from(value: toml::ser::Error) -> Self {
        ConfigWriteError::TomlSerError(value)
    }
}

impl From<std::io::Error> for ConfigWriteError {
    fn from(value: std::io::Error) -> Self {
        ConfigWriteError::IoError(value)
    }
}

#[derive(Debug)]
pub enum ConfigReadError {
    TomlDeError(toml::de::Error),
    IoError(std::io::Error),
}

impl From<toml::de::Error> for ConfigReadError {
    fn from(value: toml::de::Error) -> Self {
        ConfigReadError::TomlDeError(value)
    }
}

impl From<std::io::Error> for ConfigReadError {
    fn from(value: std::io::Error) -> Self {
        ConfigReadError::IoError(value)
    }
}
