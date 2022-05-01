pub struct Config {
    pub port: u16,
}

#[derive(Debug)]
pub enum ConfigError {
    UnErrorDeConfig,
}

impl Config {
    pub fn new() -> Result<Config, ConfigError> {
        Ok(Config { port: 3000 })
    }
}
