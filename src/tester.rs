use colored::Colorize;
use core::fmt;
use inquire::Text;
use spinners::{Spinner, Spinners};
use std::process::{exit, Command};

use serde::{Deserialize, Serialize};

use crate::{
    config::{fmt_single_parameter, Args},
    logger::log_error,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub tester: String,
    pub tester_args: Option<Args>,
}

impl Test {
    pub fn init() -> Test {
        let mut test_config = Test {
            tester: String::from(""),
            tester_args: None,
        };
        test_config.tester = Text::new("Tester program:").prompt().unwrap();
        let tester_args = Text::new("Tester args:")
            .with_help_message("<esc> to skip")
            .prompt_skippable()
            .unwrap();
        test_config.tester_args =
            tester_args.map(|args| Args(args.split_whitespace().map(|v| v.to_string()).collect()));
        test_config
    }
    pub fn run(&self) {
        let output = Command::new("which")
            .arg(&self.tester)
            .output()
            .expect("cannot run which");

        if !output.status.success() {
            log_error(
                &format!(
                    "Command `{}` not found, make sure it is installed",
                    &self.tester
                ),
                true,
            );
        }

        let binding = Vec::new();
        let args: &Vec<String> = match &self.tester_args {
            Some(args) => &args.0,
            None => &binding,
        };

        let mut spinner = Spinner::new(Spinners::Dots, "Testing in progress".into());
        let output = Command::new(&self.tester).args(args).output().expect("");
        if !output.status.success() {
            let sout = String::from_utf8(output.stdout).expect("Not UTF-8");
            let serr = String::from_utf8(output.stderr).expect("Not UTF-8");
            println!("{}", sout);
            println!("{}", serr);
        }
        let success = output.status.success();

        if success {
            spinner.stop_and_persist(&format!("{}", "".green()), "Testing OK".into());
        } else {
            spinner.stop_and_persist(&format!("{}", "".red()), "Testing failed".into());
            exit(1);
        }
    }
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = String::from("");
        match &self.tester_args {
            Some(args) => fmt_args.push_str(&format!("{}", args)),
            None => fmt_args.push_str("None"),
        }
        write!(
            f,
            "  - Command: {}\n  - Command Args: {}\n",
            fmt_single_parameter(&self.tester),
            fmt_args
        )
    }
}
