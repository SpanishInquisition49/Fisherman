use colored::Colorize;
use core::fmt;
use inquire::{Confirm, Text};
use regex::bytes::Regex;
use spinners::{Spinner, Spinners};
use std::process::{exit, Command};

use crate::config::{fmt_single_parameter, Cmd};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lint {
    pub command: Cmd,
    pub file_ext: String,
    pub single_file: bool,
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = String::from("");
        match &self.command.args {
            Some(args) => fmt_args.push_str(&format!("{}", args)),
            None => fmt_args.push_str("None"),
        }
        write!(
            f,
            "  - Command: {}\n  - File Extension: {}\n  - Run only on edited files: {}\n  - Command Args: {}\n",
            fmt_single_parameter(&self.command.name), fmt_single_parameter(&self.file_ext), self.single_file.to_string().purple(), fmt_args
        )
    }
}

impl Lint {
    pub fn init() -> Lint {
        let command = Cmd::init("Linter program:");
        let file_ext = Text::new("File extension regex").prompt().unwrap();
        let single_file = Confirm::new("Run only on modified files?")
            .with_help_message("y/n")
            .prompt()
            .unwrap();
        Lint {
            command,
            file_ext,
            single_file,
        }
    }

    pub fn run(&self) {
        if !self.command.check() {
            eprintln!(
                "Fisherman Error: Linter `{}` not found.",
                &self.command.name
            );
            exit(1);
        }
        let mut spinner = Spinner::new(Spinners::Dots, "Fisherman: Linting in progress".into());
        let (success, sout, serr) = if self.single_file {
            self.lint_file()
        } else {
            self.lint_project()
        };

        if success {
            spinner.stop_and_persist(&format!("{}", "".green()), "Fisherman: Linting OK".into());
        } else {
            spinner.stop_and_persist(&format!("{}", "".red()), "Fisherman: Linting Failed".into());
            eprintln!("{}", sout);
            eprintln!("{}", serr);
            exit(1);
        }
    }

    fn lint_file(&self) -> (bool, String, String) {
        let mut res = true;
        let mut sout = String::from("");
        let mut serr = String::from("");
        let staged_files = Command::new("git")
            .arg("diff")
            .arg("--cached")
            .arg("--name-only")
            .output()
            .expect("git cannot be executed");
        let staged_files = String::from_utf8(staged_files.stdout).unwrap();
        let staged_files = staged_files.split('\n');
        let re = Regex::new(&self.file_ext).unwrap();
        for file in staged_files {
            if re.is_match(file.as_bytes()) {
                eprintln!("{}", file);
                let binding = Vec::new();
                let args: &Vec<String> = match &self.command.args {
                    Some(args) => &args.0,
                    None => &binding,
                };

                let output = Command::new(&self.command.name)
                    .args(args)
                    .arg(file)
                    .output()
                    .expect("");
                if !output.status.success() {
                    res = false;
                    sout = String::from_utf8(output.stdout).expect("Not UTF-8");
                    serr = String::from_utf8(output.stderr).expect("Not UTF-8");
                }
            }
        }
        (res, sout, serr)
    }

    fn lint_project(&self) -> (bool, String, String) {
        let binding = Vec::new();
        let mut sout = String::from("");
        let mut serr = String::from("");
        let args: &Vec<String> = match &self.command.args {
            Some(args) => &args.0,
            None => &binding,
        };

        let output = Command::new(&self.command.name)
            .args(args)
            .output()
            .expect("");
        if !output.status.success() {
            sout = String::from_utf8(output.stdout).expect("Not UTF-8");
            serr = String::from_utf8(output.stderr).expect("Not UTF-8");
        }
        (output.status.success(), sout, serr)
    }
}
