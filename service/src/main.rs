use clap::Parser;
use config::config::parse_config;

use io_uring::IoUring;
use tcp_loop::tcp_loop::tcp_loop;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Path to the config file")]
    config_path: String,
}

fn main() {
    // Parse cli
    let args = Args::parse();

    // Parse config
    let conf = match parse_config(&args.config_path) {
        Ok(ca) => ca,
        Err(err) => {
            println!("Error parsing config: {}", err.message);
            return;
        }
    };

    println!("{:?}", conf);

    println!("Starting yottafs...");

    // Init io driver

    // Start service

    let ring = match IoUring::new(128) {
        Ok(ring) => ring,
        Err(err) => {
            println!("Error creating io_uring: {}", err);
            return;
        }
    };

    // Pick a loop
    let event_loop = tcp_loop;

    match event_loop(ring) {
        Ok(_) => println!("event loop exited successfully"),
        Err(err) => println!("event loop exited with error: {}", err),
    }
}
