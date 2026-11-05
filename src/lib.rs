use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fmt, fs,
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

pub fn save_user_config(cfg: &UserConfig, path: &PathBuf) -> Result<(), ConfigWriteError> {
    let toml_str = toml::to_string(&cfg)?;

    std::fs::write(&path, &toml_str)?;

    Ok(())
}

pub fn flushed_print(str: &str) {
    print!("{}", str);
    std::io::stdout().flush().unwrap();
}

pub fn read_line_as_string() -> Result<String, io::Error> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input)
}

pub fn establish_connection(ip: &str) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
    let url = format!(
        "http://{}/notify.ps3mapi?msg=Rich+Presence+connection+established&icon=0&snd=",
        ip
    );

    let response = ureq::get(url).call()?;

    return Ok(response);
}

// TODO: Replace query_game() with this new version that uses cpursx.ps3
pub fn query_game_cpursx(ip: &str) -> Result<Option<CurrentGame>, ureq::Error> {
    let url = format!("http://{}/cpursx.ps3", ip);
    todo!("Finish query_game_cpursx()");
    Ok(Some(CurrentGame::new("test", "test")))
}

pub fn query_game(ip: &str) -> Result<Option<CurrentGame>, ureq::Error> {
    let url = format!("http://{}/klic.ps3", ip);

    // Get the raw klic page from the PS3
    let page: String = match ureq::get(url).call() {
        Ok(response) => response.into_body().read_to_string()?,
        Err(err) => todo!("{err}\nHandle ureq::get(ip).call() failure"),
    };

    let current_game: Option<CurrentGame>;

    // Parse the page, returning either Some or None
    if page.contains("KLIC: Not in-game!") {
        current_game = None;
    } else {
        let (mut trimmed_page, _) = page.split_once("</H2>").unwrap();
        (_, trimmed_page) = trimmed_page.split_once("<h2>").unwrap();

        if trimmed_page.len() > 1 {
            let (id, game) = trimmed_page.split_at(10);
            current_game = Some(CurrentGame::new(id.trim_end(), game));
        } else {
            current_game = None;
        }
    }

    return Ok(current_game);
}

pub fn any_key_continue() {
    flushed_print("Press enter to continue.");
    let _ = io::stdin().read_line(&mut String::new());
}

pub fn die(msg: &str) -> ! {
    println!("{msg}");
    any_key_continue();
    std::process::exit(1);
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

impl fmt::Display for ConfigWriteError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self}")
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
