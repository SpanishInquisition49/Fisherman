use colored::Colorize;
use core::fmt;
use inquire::MultiSelect;
use serde::{Deserialize, Serialize};

use crate::{commit_message::CommitMessage, logger::log_error, pre_commit::PreCommit};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub pre_commit: Option<PreCommit>,
    pub commit_message: Option<CommitMessage>,
}

impl Config {
    pub fn init() -> Config {
        let mut config: Config = Config {
            pre_commit: None,
            commit_message: None,
        };
        let config_options = vec!["Pre Commit", "Commit Message"];
        let answers = match MultiSelect::new("Select the hooks to enable:", config_options).prompt()
        {
            Ok(res) => res,
            Err(e) => {
                log_error(&e.to_string(), true);
                unreachable!();
            }
        };
        for option in answers {
            match option {
                "Pre Commit" => config.pre_commit = Some(PreCommit::init()),
                "Commit Message" => config.commit_message = Some(CommitMessage::init()),
                _ => (),
            }
        }
        config
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt: String = String::from("");
        match &self.pre_commit {
            Some(pre_commit) => {
                fmt.push_str(&format!("[{}]\n{}", "Pre Commit".bold().blue(), pre_commit))
            }
            None => (),
        };
        match &self.commit_message {
            Some(commit_message) => fmt.push_str(&format!(
                "[{}]\n{}",
                "Commit Message".bold().blue(),
                commit_message
            )),
            None => (),
        }
        write!(f, "{}", fmt)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Args(pub Vec<String>);

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = String::from("");
        for (index, arg) in self.0.iter().enumerate() {
            if index == &self.0.len() - 1 {
                fmt_args.push_str(&fmt_single_parameter(arg))
            } else {
                fmt_args.push_str(&format!("{}, ", fmt_single_parameter(arg)));
            }
        }
        write!(f, "{}{}{}", "[".red(), fmt_args, "]".red())
    }
}

pub fn fmt_single_parameter(parameter: &str) -> String {
    format!("{}{}{}", "\"".yellow(), parameter.yellow(), "\"".yellow())
}
