use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: ConfigServer,
}

#[derive(Deserialize, Debug)]
pub struct ConfigServer {
    pub port: u32,
}
