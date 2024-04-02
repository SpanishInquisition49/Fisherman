mod config;
use crate::config::Config;
use std::fs;

const CONFIG_FILE: &str = ".fishermanrc.toml";

fn main() {
    // Get Toml config
    let config: Option<Config> = match fs::read_to_string(CONFIG_FILE) {
        Ok(result) => Some(toml::from_str(&result).unwrap()),
        Err(_) => None,
    };

    match config {
        Some(c) => {
            specs(c);
        }
        None => {
            init();
        }
    };
}

fn init() {
    println!("Welcome to fisherman, your git hooks manager!");
    // TODO: Add .fishermanrc.toml generation
}

fn specs(config: Config) {
    println!("Fisherman features:\n{}", config.to_string());
}
