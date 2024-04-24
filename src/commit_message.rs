use colored::Colorize;
use core::fmt;
use inquire::{Confirm, Text};

use crate::config::{fmt_single_parameter, Args};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitMessage {
    pub validation_command: String,
    pub validation_command_args: Option<Args>,
    pub retryable: bool,
    pub template_path: Option<String>,
}

impl CommitMessage {
    pub fn init() -> CommitMessage {
        let mut commit_message = CommitMessage {
            validation_command: String::from(""),
            validation_command_args: None,
            retryable: false,
            template_path: None,
        };

        commit_message.validation_command = Text::new("Validation program:").prompt().unwrap();
        let validation_args = Text::new("Program args:")
            .with_help_message("<esc> to skip")
            .prompt_skippable()
            .unwrap();
        commit_message.validation_command_args = validation_args
            .map(|args| Args(args.split_whitespace().map(|v| v.to_string()).collect()));
        commit_message.retryable = Confirm::new("Enable message fix?")
            .with_help_message("y/n")
            .prompt()
            .unwrap();
        commit_message.template_path = Text::new("Commit Message template path:")
            .with_help_message("<esc> to skip")
            .prompt_skippable()
            .unwrap();
        commit_message
    }

    pub fn run(&self) {
        // TODO
    }
}

impl fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args: String = String::from("");
        match &self.validation_command_args {
            Some(args) => fmt_args.push_str(&format!("{}", args)),
            None => fmt_args.push_str("None"),
        }
        let template_path: String = match &self.template_path {
            Some(path) => path.clone(),
            None => String::from("None"),
        };
        write!(f, " - Validation Command: {}\n - Command Args: {}\n - Keep Message on Fail: {}\n - Template Path: {}", 
                fmt_single_parameter(&self.validation_command),
                fmt_args,
                self.retryable.to_string().purple(),
                fmt_single_parameter(&template_path)
        )
    }
}
