use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    error::Error, io::{self, Write}, path::PathBuf, time::Duration
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

pub fn create_empty_config() -> Result<(), Box<dyn Error>> {
    Ok(())
}

pub fn load_user_config(path: &PathBuf) -> Result<UserConfig, ConfigReadError> {
    let cfg_contents = std::fs::read_to_string(&path)?;
    /*
    let cfg_contents = match fs::read_to_string(&config_file_path) {
        Ok(contents) => contents,
        Err(_) => {
            println!("No config file found.");

            // Create parent directories
            let p = config_file_path.parent().unwrap();
            fs::create_dir_all(p)?;

            let mut ip: Box<str>;

            loop {
                // Get PS3 IP as trimmed, boxed str
                print_flush_buffer("Please enter PS3 IP: ");
                ip = Box::from(read_line_as_string().trim());

                // Shortest possible IP is 7 characters long
                if ip.len() > 7 {
                    break;
                }
            }

            // Create new settings struct with user-specified IP
            user_config = UserConfig::new(&ip, Duration::from_secs(10), Duration::from_secs(20));

            // Create new config file
            File::create(&config_file_path).unwrap();

            // Write new user_config struct to file for later
            let toml_str = toml::to_string(&user_config).unwrap();
            if let Err(err) = std::fs::write(config_file_path, &toml_str) {
                println!("Failed to write config file: {err}");
                any_key_continue();
                process::exit(1);
            }

            // Return the toml string as we parse it later
            toml_str
        }
    };
    */
    let cfg = toml::from_str(&cfg_contents)?;

    Ok(cfg)
    /*
    let user_config: UserConfig = match toml::from_str(&cfg_contents) {
        Ok(contents) => contents,
        Err(err) => {
            println!("Failed to read settings.toml: {err}");
            any_key_continue();
            process::exit(1);
        }
    };
    */
}

pub fn save_user_config(cfg: &UserConfig) {
    // TODO
}

pub fn print_flush_buffer(str: &str) {
    print!("{}", str);
    std::io::stdout().flush().unwrap();
}

pub fn read_line_as_string() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    return input;
}

pub fn any_key_continue() {
    print_flush_buffer("Press enter to continue.");
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
