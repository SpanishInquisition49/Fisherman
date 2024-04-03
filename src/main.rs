mod config;
mod linter;
mod tester;
use crate::config::Config;
use crate::linter::lint;
use crate::tester::test;
use std::env;
use std::fs;
use std::process::exit;

const CONFIG_FILE: &str = ".fishermanrc.toml";

fn main() {
    // Get Toml config
    let config: Option<Config> = match fs::read_to_string(CONFIG_FILE) {
        Ok(result) => Some(toml::from_str(&result).unwrap()),
        Err(_) => None,
    };
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        match config {
            Some(c) => {
                specs(c);
            }
            None => {
                init();
            }
        };
    } else {
        match config {
            Some(c) => execute(&args[1], c),
            None => {
                println!("Error: no configutation found");
                exit(1);
            }
        }
    }
}

fn init() {
    println!("Welcome to fisherman, your git hooks manager!");
    // TODO: Add .fishermanrc.toml generation
}

fn specs(config: Config) {
    println!("Fisherman features:\n{}", config.to_string());
}

fn execute(action: &str, config: Config) {
    match action {
        "Linter" => lint(config.lint),
        "Test" => test(config.test),
        _ => {
            println!("Nothing to do");
        }
    }
}
