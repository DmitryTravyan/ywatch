pub(crate) mod server;

use std::net::IpAddr;
use std::str::FromStr;
use log::{info, debug};
use std::io::Read;
use serde_json::{Value, Map};
use serde::Deserialize;
use clap::ArgMatches;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct Configuration {
    #[serde(default = "String::default")]
    path: String,
    #[serde(default = "String::default")]
    ip: String,
    #[serde(default = "String::default")]
    port: String,
    #[serde(default = "Vec::<String>::default")]
    interrogation_urls: Vec<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            path: "config.json".to_string(),
            ip: "0.0.0.0".to_string(),
            port: "8080".to_string(),
            interrogation_urls: Vec::with_capacity(0),
        }
    }
}

impl Configuration {
    /// Create new configuration with params:
    /// path: config.json
    /// ip: 0.0.0.0
    /// port: 8080
    /// interrogation_urls: Vec::with_capacity(0)
    pub(crate) fn new() -> Self {
        Configuration::default()
    }

    fn parse_file(file: &Vec<u8>) -> Value {
        let configuration: Value = serde_json::from_slice(file).expect(
            "nok!"
        );
        configuration
    }

    /// Create configuration from file.
    /// If file does not exists return default Configuration
    pub(crate) fn from_file(path: &str) -> Self {
        match std::fs::File::open(path) {
            Ok(mut file) => {
                let mut buffer = Vec::<u8>::new();
                file.read_to_end(&mut buffer);
                let conf_value  = Configuration::parse_file(&buffer);
                Self {
                    path: if let Some(Value::String(path)) = conf_value.get("path") {
                        path.to_string()
                    } else {
                        "config.json".to_string()
                    },
                    ip: if let Some(Value::String(ip)) = conf_value.get("ip") {
                        ip.to_string()
                    } else {
                        "0.0.0.0".to_string()
                    },
                    port: if let Some(Value::String(port)) = conf_value.get("port") {
                        port.to_string()
                    } else {
                        "8080".to_string()
                    },
                    interrogation_urls: Vec::<String>::new()
                }
            },
            Err(e) => {
                info!("Error then reading configuration file {}. {}", path, e);
                Configuration::default()
            }
        }
    }

    /// Try to catch file path from args, and form configuration struct from it.
    pub(crate) fn from_args(args: ArgMatches) -> Self {
        if let Some(path) = args.value_of("configuration") {
            Configuration::from_file(path)
        } else {
            Configuration::from_file("config.json")
        }
    }

    pub(crate) fn ip(&self) -> &str {
        &self.ip
    }

    pub(crate) fn port(&self) -> &str {
        &self.port
    }
}
