#![allow(dead_code)]
#![allow(unused_variables)]

mod tl_client;

use clap::Parser;
use serde::Deserialize;

#[derive(Parser)]
struct Cli {
    #[arg(short='e')]
    config_file: Option<String>,

    #[arg(short='l')]
    local_addr: Option<String>,

    #[arg(short='s')]
    server_addr: Option<String>,

    #[arg(short='k')]
    sec_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct JsonConfig {
    #[serde(rename="localAddr")]
    local_addr: Option<String>,

    #[serde(rename="serverAddr")]
    server_addr: Option<String>,

    #[serde(rename="secKey")]
    sec_key: Option<String>,

    #[serde(rename="fakeRequest")]
    fake_request: Option<Vec<String>>,

    #[serde(rename="fakeResponse")]
    fake_response: Option<Vec<String>>,
}

fn main() {
    let opt_local_addr: Option<String> = None;
    let opt_server_addr: Option<String> = None;
    let opt_sec_key: Option<String> = None;

    let cli = Cli::parse();

    if let Some(config_file) = cli.config_file.as_deref() {
        let config_data: String;
        match std::fs::read_to_string(config_file) {
            Ok(v) => config_data = v,
            Err(e) => {
                eprintln!("load config file {} failed: {}", config_file, e);
                std::process::exit(1);
            }
        };
        let json_config: JsonConfig;
        match serde_json::from_str(&config_data) {
            Ok(v) => json_config = v,
            Err(e) => {
                eprintln!("parse config file {} failed: {}", config_file, e);
                std::process::exit(1);
            }
        }
    }
}
