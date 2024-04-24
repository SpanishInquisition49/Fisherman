use colored::Colorize;
use core::fmt;
use inquire::{Confirm, Text};
use regex::bytes::Regex;
use spinners::{Spinner, Spinners};
use std::process::{exit, Command};

use crate::config::{fmt_single_parameter, Args};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Lint {
    pub linter: String,
    pub file_ext: String,
    pub single_file: bool,
    pub linter_args: Option<Args>,
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = String::from("");
        match &self.linter_args {
            Some(args) => fmt_args.push_str(&format!("{}", args)),
            None => fmt_args.push_str("None"),
        }
        write!(
            f,
            "  - Command: {}\n  - File Extension: {}\n  - Run only on edited files: {}\n  - Command Args: {}\n",
            fmt_single_parameter(&self.linter), fmt_single_parameter(&self.file_ext), self.single_file.to_string().purple(), fmt_args
        )
    }
}

impl Lint {
    pub fn init() -> Lint {
        let mut lint_config = Lint {
            linter: String::from(""),
            file_ext: String::from(""),
            single_file: true,
            linter_args: None,
        };
        lint_config.linter = Text::new("Linter program:").prompt().unwrap();
        lint_config.file_ext = Text::new("File extension regex").prompt().unwrap();
        lint_config.single_file = Confirm::new("Run only on modified files?")
            .with_help_message("y/n")
            .prompt()
            .unwrap();
        let linter_args = Text::new("Linter args:")
            .with_help_message("<esc> to skip")
            .prompt_skippable()
            .unwrap();
        lint_config.linter_args =
            linter_args.map(|args| Args(args.split_whitespace().map(|v| v.to_string()).collect()));
        lint_config
    }

    pub fn run(&self) {
        let output = Command::new("which")
            .arg(&self.linter)
            .output()
            .expect("cannot run which");

        if !output.status.success() {
            println!("Fisherman Error: Linter `{}` not found.", &self.linter);
            exit(1);
        }
        let mut spinner = Spinner::new(Spinners::Dots, "Linting in progress".into());
        let success = if self.single_file {
            self.lint_file()
        } else {
            self.lint_project()
        };

        if success {
            spinner.stop_and_persist(&format!("{}", "".green()), "Linting OK".into());
        } else {
            spinner.stop_and_persist(&format!("{}", "".red()), "Linting failed".into());
            exit(1);
        }
    }

    fn lint_file(&self) -> bool {
        let mut res = true;
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
                println!("{}", file);
                let binding = Vec::new();
                let args: &Vec<String> = match &self.linter_args {
                    Some(args) => &args.0,
                    None => &binding,
                };

                let output = Command::new(&self.linter)
                    .args(args)
                    .arg(file)
                    .output()
                    .expect("");
                if !output.status.success() {
                    res = false;
                    let sout = String::from_utf8(output.stdout).expect("Not UTF-8");
                    let serr = String::from_utf8(output.stderr).expect("Not UTF-8");
                    println!("{}", sout);
                    println!("{}", serr);
                }
            }
        }
        res
    }

    fn lint_project(&self) -> bool {
        let binding = Vec::new();
        let args: &Vec<String> = match &self.linter_args {
            Some(args) => &args.0,
            None => &binding,
        };

        let output = Command::new(&self.linter).args(args).output().expect("");
        if !output.status.success() {
            let sout = String::from_utf8(output.stdout).expect("Not UTF-8");
            let serr = String::from_utf8(output.stderr).expect("Not UTF-8");
            println!("{}", sout);
            println!("{}", serr);
        }
        output.status.success()
    }
}
