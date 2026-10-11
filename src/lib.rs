use clap::Parser;
use std::time::Duration;

#[derive(Parser, Debug)]
pub struct Args {
    pub ps3_ip: Box<str>,
    pub sleep_duration: Option<u64>,
    pub reconnect_delay: Option<u64>,
}

#[derive(Debug)]
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
