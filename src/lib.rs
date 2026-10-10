use clap::Parser;
use std::time::Duration;

#[derive(Debug)]
pub struct UserConfig {
    pub ps3_ip: String,
    pub sleep_duration: Duration,
}

impl UserConfig {
    pub fn new(ps3_ip: &str, sleep_duration: Duration) -> Self {
        return UserConfig { ps3_ip: ps3_ip.to_string(), sleep_duration };
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

#[derive(Parser, Debug)]
pub struct Args {
    pub ps3_ip: String,
}
