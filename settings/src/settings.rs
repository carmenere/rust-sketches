#![allow(deprecated)]
use config::{Config, Environment, ConfigError};
use serde::Deserialize;
use std::error::Error;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub port: u16,
}

impl Server {
    pub fn new() -> Self{
        Self {
            port: env::var("SERVER_PORT").unwrap().parse::<u16>().unwrap()
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub topvar: String,
}

impl Settings {
    pub fn new() -> Self {
        let server = Server::new();
        let topvar = env::var("TOPVAR").unwrap();
        Self {
            server: server,
            topvar: topvar
        }
    }
}