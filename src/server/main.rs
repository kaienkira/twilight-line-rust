#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use clap::Parser as ClapParser;
use std::sync::LazyLock;

mod proxy;
mod server_error;
mod tl_server;

struct Config {
    local_addr: String,
    sec_key: String,
    fake_request: String,
    fake_response: String,
}

#[derive(ClapParser)]
struct Cli {
    #[arg(short = 'e', help = "config file path")]
    config_file: Option<String>,

    #[arg(short = 'l', help = "local listen addr")]
    local_addr: Option<String>,

    #[arg(short = 'k', help = "secure key")]
    sec_key: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
struct JsonConfig {
    #[serde(rename = "localAddr")]
    local_addr: Option<String>,

    #[serde(rename = "secKey")]
    sec_key: Option<String>,

    #[serde(rename = "fakeRequest")]
    fake_request: Option<Vec<String>>,

    #[serde(rename = "fakeResponse")]
    fake_response: Option<Vec<String>>,
}

fn parse_config() -> Config {
    let mut opt_local_addr: Option<String> = None;
    let mut opt_sec_key: Option<String> = None;
    let mut opt_fake_request: Option<String> = None;
    let mut opt_fake_response: Option<String> = None;

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

        opt_local_addr = json_config.local_addr;
        opt_sec_key = json_config.sec_key;

        if let Some(fake_request) = json_config.fake_request {
            opt_fake_request = Some(fake_request.join(""));
        }
        if let Some(fake_response) = json_config.fake_response {
            opt_fake_response = Some(fake_response.join(""));
        }
    }

    if cli.local_addr.is_some() {
        opt_local_addr = cli.local_addr;
    }
    if cli.sec_key.is_some() {
        opt_sec_key = cli.sec_key;
    }

    let mut check_opt_result = true;
    loop {
        if opt_local_addr.is_none() {
            check_opt_result = false;
            eprintln!("config.localAddr is required");
            break;
        }
        break;
    }
    if check_opt_result == false {
        eprintln!("please read help with -h or --help");
        std::process::exit(1);
    }

    Config {
        local_addr: opt_local_addr.unwrap(),
        sec_key: opt_sec_key.unwrap_or(String::new()),
        fake_request: opt_fake_request.unwrap_or(String::new()),
        fake_response: opt_fake_response.unwrap_or(String::new()),
    }
}

fn build_tokio_runtime() -> tokio::runtime::Runtime {
    match tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .build()
    {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("build tokio runtime failed: {}", e);
            std::process::exit(1);
        }
    }
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| parse_config());

fn main() {
    let config = &*CONFIG;
    let rt = build_tokio_runtime();
    if let Err(e) = rt.block_on(proxy::handle_proxy(config)) {
        eprintln!("handle proxy failed: {}", e);
        std::process::exit(1);
    }
}
