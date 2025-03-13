#![allow(dead_code)]
#![allow(unused_variables)]

mod tl_client;

use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short='e')]
    config_file: Option<String>,

    #[arg(short='l')]
    local_addr: Option<String>,

    #[arg(short='s')]
    server_addr: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(config_file) = cli.config_file.as_deref() {
        let config_data = match std::fs::read_to_string(config_file) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("load config file {} failed: {}", config_file, e);
                std::process::exit(1);
            }
        };
    }
}
