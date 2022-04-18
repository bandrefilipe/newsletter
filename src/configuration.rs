use std::net::{TcpListener, ToSocketAddrs};
use std::{env, io};

use config::{ConfigError, Environment, File};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

pub fn parse() -> Result<ApplicationConfig, ConfigError> {
    let environment = env::var("ENV").unwrap_or_else(|_| "local".into());

    config::Config::builder()
        .add_source(File::with_name("configuration/default"))
        .add_source(File::with_name(&format!("configuration/{environment}")).required(false))
        .add_source(Environment::with_prefix("APP"))
        .build()?
        .try_deserialize()
        .map(print_debug) // using `map` here to print state (if debug enabled) while `inspect` is unstable
}

fn print_debug(cfg: ApplicationConfig) -> ApplicationConfig {
    if cfg.debug {
        tracing::info!("Configuration debug is enabled! {:?}", cfg);
    }
    cfg
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub debug: bool,
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
    pub migrate: bool,
    pub user: String,
    pub password: Secret<String>,
    pub host: String,
    pub port: u16,
    pub dbname: String,
}

impl DatabaseConfig {
    pub fn connection_string(&self) -> Secret<String> {
        let password = self.password.expose_secret();
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user, password, self.host, self.port, self.dbname
        ))
    }

    /// Intended for tests, so we can connect to no specific logical database.
    /// (useful for test isolation, since we can create a new database for each test)
    pub fn connection_string_without_db(&self) -> Secret<String> {
        let password = self.password.expose_secret();
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.user, password, self.host, self.port
        ))
    }
}
