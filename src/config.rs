use config::{ConfigError, Environment};
use serde::Deserialize;
use std::net::{TcpListener, ToSocketAddrs};
use std::{env, io};

pub fn parse() -> Result<ApplicationConfig, ConfigError> {
    let environment = env::var("ENV").unwrap_or_else(|_| "local".into());

    config::Config::builder()
        .add_source(config::File::with_name("config/default"))
        .add_source(config::File::with_name(&format!("config/{environment}")).required(false))
        .add_source(Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
}

impl ServerConfig {
    pub fn local_address(&self) -> impl ToSocketAddrs {
        format!("localhost:{}", self.port)
    }

    pub fn listener(&self) -> io::Result<TcpListener> {
        TcpListener::bind(self.local_address())
    }
}

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, self.password, self.host, self.port, self.dbname
        )
    }
}
