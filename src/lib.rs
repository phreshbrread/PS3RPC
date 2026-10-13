use serde::{Deserialize, Serialize};
use std::{env, io::Write, path::PathBuf, time::Duration};

// #[derive(Parser, Debug)]
// pub struct Args {
//     pub ps3_ip: Box<str>,
//     pub sleep_duration: Option<u64>,
//     pub reconnect_delay: Option<u64>,
// }

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

pub fn get_config_path() -> PathBuf {
    todo!("Use platform-native config path");
    let path = PathBuf::from(
        env::home_dir()
            .unwrap() // $HOME should always be set on Linux
            .join(env::var("XDG_CONFIG_HOME").unwrap_or_else(|err| {
                println!("Failed to read $XDG_CONFIG_HOME: {}", err);
                println!("Defaulting to ~/.config");
                String::from(".config")
            }))
            .join("ps3rpc")
            .join("settings.toml"),
    );

    return path;
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
