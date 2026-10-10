use std::time::Duration;

pub struct UserConfig {
    pub ps3_ip: &'static str,
    pub sleep_duration: Duration,
}

impl UserConfig {
    pub fn testing_defaults() -> Self {
        UserConfig {
            ps3_ip: "192.168.20.8",
            sleep_duration: Duration::from_secs(10),
        }
    }
}
