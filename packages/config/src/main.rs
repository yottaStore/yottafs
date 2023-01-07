use config::config::parse_config;
use std::env;

fn main() {
    println!("Hello, world!");
    println!("Current dir: {:?}", env::current_dir());

    //let file_path = "/home/mamluk/Projects/yotta/yottafs/packages/config/config.json";
    let file_path = "./e2e/config.json";
    let c = parse_config(file_path).unwrap();

    println!("config: {:?}", c);
}
