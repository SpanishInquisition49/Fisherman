mod commit_message;
mod config;
mod linter;
mod logger;
mod pre_commit;
mod tester;
mod flags;
use config::Config;
use std::str::FromStr;
use logger::log_error;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use flags::Flags;

use crate::pre_commit::PreCommit;


const CONFIG_FILE: &str = ".fisherman.toml";

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
                specs(&c);
            }
            None => {
                init();
            }
        };
    } else {
        match config {
            Some(c) => execute(args, c),
            None => {
                log_error("no configutation found", true);
            }
        }
    }
}

fn init() {
    eprintln!("Welcome to fisherman, your git hooks manager!");
    if !Path::new("./.git/").exists() {
        log_error("git is not initialized in this directory", true);
    }

    let config = Config::init();
    let toml = toml::to_string(&config);
    match toml {
        Ok(mut toml) => {
            let mut file = File::create(CONFIG_FILE).unwrap();
            eprintln!("{}", config);
            unsafe {
                match file.write_all(toml.as_bytes_mut()) {
                    Ok(_) => eprintln!("Configuration created!"),
                    Err(e) => {
                        log_error(&e.to_string(), true);
                    }
                };
            };
        }
        Err(e) => {
            log_error(&e.to_string(), true);
        }
    }
}

fn specs(config: &Config) {
    eprintln!("Fisherman features:\n{}", config);
}

fn execute(args: Vec<String>, config: Config) {
    let pre_commit = config.pre_commit.clone().unwrap_or(PreCommit {
        lint: None,
        test: None,
    });
    env::set_current_dir(&config.root_directory).unwrap();
    let action: Flags = Flags::from_str(&args[1]).unwrap();
    match action {
        Flags::Init => init(),
        Flags::Lint => {
            if pre_commit.lint.is_some() {
                pre_commit.lint.unwrap().run();
            } else {
                log_error("lint config not found", true);
            }
        }
        Flags::Test => {
            if pre_commit.test.is_some() {
                pre_commit.test.unwrap().run();
            } else {
                log_error("test config not found", true);
            }
        }
        Flags::PreCommit => {
            pre_commit.run();
        }
        Flags::CommitMessage => {
            let commit_message_path = &args[2.. args.len()];
            if config.commit_message.is_some() {
                config.commit_message.unwrap().run(Some(&commit_message_path.join(" ")), None);
            }
        }
        Flags::ShowConfig => {
            specs(&config)
        },

        Flags::ApplyHooks => config.apply(),
        Flags::Help => (),
        _ => {
            eprintln!("Nothing to do");
        }
    }
}
