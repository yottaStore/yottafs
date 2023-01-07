mod tcp_loop;

use clap::Parser;
use config::config::parse_config;
use std::error::Error;

use io_uring::IoUring;
use tcp_loop::tcp_loop;

#[derive(Parser)]
struct CliArgs {
    #[arg(default_value = "/home/mamluk/Projects/yotta/yottafs/e2e/config.json")]
    config_path: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting yottafs...");

    // Parse cli
    let args = CliArgs::parse();

    // Parse config
    let ca = parse_config(&args.config_path).unwrap();

    println!("{:?}", ca);

    // Init io driver

    // Start service

    let ring = IoUring::new(128)?;

    tcp_loop(ring)
}
