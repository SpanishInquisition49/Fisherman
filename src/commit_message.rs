use colored::Colorize;
use core::fmt;
use inquire::{Confirm, Editor, Text};
use spinners::{Spinner, Spinners, Stream};
use std::{
    fs::{File}, io::Read, process::{exit, Command}
};

use crate::config::{fmt_single_parameter, Cmd};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommitMessage {
    pub validation_command: Cmd,
    pub retryable: bool,
    pub template_path: Option<String>,
}

impl CommitMessage {
    pub fn init() -> CommitMessage {
        let validation_command = Cmd::init("Validation program:");
        let retryable = Confirm::new("Enable message fix?")
            .with_help_message("y/n")
            .prompt()
            .unwrap();
        let template_path = Text::new("Commit Message template path:")
            .with_help_message("<esc> to skip")
            .prompt_skippable()
            .unwrap();
        CommitMessage {
            validation_command,
            retryable,
            template_path,
        }
    }

    pub fn run(&self, path: Option<&str>, message: Option<&str>) {
        // Check if the validation command exist
        if !self.validation_command.check() {
            eprintln!(
                "Fisherman Error: Command `{}` not found.",
                &self.validation_command.name
            );
            exit(1)
        }

        // Decide which option use path is preferred,
        // but in case of re-run this function use
        // the old invalid commit message
        let message = match (path, message) {
            (Some(p), _) => {
                let mut file = File::open(p).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                content
            }
            (None, Some(m)) => String::from(m),
            (None, None) => String::from(""),
        };

        // If the command take some args the message should
        // be placed in correct position
        let binding = vec![String::from("message")];
        let mut args: Vec<String> = match &self.validation_command.args {
            Some(args) => args.0.clone(),
            None => binding,
        };

        let mut replaced = false;
        args.iter_mut().for_each(|arg| {
            if arg == "message" {
                *arg = String::from(&message);
                replaced = true;
            }
        });

        if !replaced {
            eprintln!("Fisherman Error: `message` args not found.",);
            exit(1);
        }

        let mut spinner = Spinner::with_stream(
            Spinners::Dots,
            "Fisherman: Commit Message Validation".into(),
            Stream::Stderr,
        );
        let output = Command::new(&self.validation_command.name)
            .args(args)
            .output()
            .expect("");
        let success = output.status.success();

        if success {
            spinner.stop_and_persist(
                &format!("{}", "".green()),
                "Fisherman: Commit Message OK".into(),
            );
        } else {
            spinner.stop_and_persist(
                &format!("{}", "".red()),
                "Fisherman: Commit Message Validation Failed".into(),
            );
            let sout = String::from_utf8(output.stdout).expect("Not UTF-8");
            let serr = String::from_utf8(output.stderr).expect("Not UTF-8");
            eprintln!("{}", sout);
            eprintln!("{}", serr);
            if self.retryable {
                self.retry(&message);
            }
            exit(1);
        }
        println!("{}", CommitMessage::format_message(&message));
    }

    fn retry(&self, old_message: &str) {
        let new_message = Editor::new("New Commit Message:")
            .with_predefined_text(old_message)
            .prompt()
            .unwrap();
        self.run(None, Some(&new_message));
    }

    fn format_message(message: &str) -> String {
        message
            .split('\n')
            .filter(|&line| !line.replace(' ', "").starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl fmt::Display for CommitMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args: String = String::from("");
        match &self.validation_command.args {
            Some(args) => fmt_args.push_str(&format!("{}", args)),
            None => fmt_args.push_str(&format!("{}", "\"None\"".yellow())),
        }
        let template_path: String = match &self.template_path {
            Some(path) => path.clone(),
            None => String::from("None"),
        };
        write!(f, " - Validation Command: {}\n - Command Args: {}\n - Keep Message on Fail: {}\n - Template Path: {}", 
                fmt_single_parameter(&self.validation_command.name),
                fmt_args,
                self.retryable.to_string().purple(),
                fmt_single_parameter(&template_path)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::CommitMessage;

    #[test]
    fn test_format_message() {
        let formatted = CommitMessage::format_message("foo\nbar\n#foobar");
        assert_eq!("foo\nbar", formatted)
    }
}
