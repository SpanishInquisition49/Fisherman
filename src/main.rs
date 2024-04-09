mod config;
mod linter;
mod tester;
use crate::config::Config;
use crate::linter::{lint, lint_init};
use crate::tester::{test, test_init};
use inquire::MultiSelect;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
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
    if !Path::new("./.git/").exists() {
        println!("git is not initialized in this directory");
        exit(1);
    }
    let mut config: Config = Config {
        lint: None,
        test: None,
    };
    let options = vec!["Linting", "Testing"];
    let ans = match MultiSelect::new("Select the feature to enable:", options).prompt() {
        Ok(res) => res,
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    };
    for feature in ans {
        match feature {
            "Linting" => {
                config.lint = Some(lint_init());
            }
            "Testing" => {
                config.test = Some(test_init());
            }
            _ => (),
        };
    }
    let toml = toml::to_string(&config);
    match toml {
        Ok(mut toml) => {
            println!("{toml}");
            let mut file = File::create(CONFIG_FILE).unwrap();
            unsafe {
                match file.write_all(toml.as_bytes_mut()) {
                    Ok(_) => println!("Configuration created!"),
                    Err(e) => {
                        println!("Error: {e}");
                        exit(1);
                    }
                };
            };
        }
        Err(e) => {
            println!("{e}");
            exit(1);
        }
    }
}

fn specs(config: Config) {
    println!("Fisherman features:\n{}", config);
}

fn execute(action: &str, config: Config) {
    match action {
        "init" => init(),
        "linter" => lint(config.lint),
        "test" => test(config.test),
        _ => {
            println!("Nothing to do");
        }
    }
}
