use colored::Colorize;
use core::fmt;
use spinners::{Spinner, Spinners};
use std::process::{exit, Command};

use serde::{Deserialize, Serialize};

use crate::{
    config::{fmt_single_parameter, Cmd},
    logger::log_error,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Test {
    pub command: Cmd,
}

impl Test {
    pub fn init() -> Test {
        let command = Cmd::init("Tester program:");
        Test { command }
    }

    pub fn run(&self) {
        if !self.command.check() {
            log_error(
                &format!(
                    "Command `{}` not found, make sure it is installed",
                    &self.command.name
                ),
                true,
            );
        }

        let binding = Vec::new();
        let args: &Vec<String> = match &self.command.args {
            Some(args) => &args.0,
            None => &binding,
        };

        let mut spinner = Spinner::with_stream(Spinners::Dots, "Fisherman: Testing in progress".into(), spinners::Stream::Stderr);
        let output = Command::new(&self.command.name)
            .args(args)
            .output()
            .expect("");
        let success = output.status.success();

        if success {
            spinner.stop_and_persist(&format!("{}", "".green()), "Fisherman: Testing OK".into());
        } else {
            spinner.stop_and_persist(&format!("{}", "".red()), "Fisherman: Testing Failed".into());
            let sout = String::from_utf8(output.stdout).expect("Not UTF-8");
            let serr = String::from_utf8(output.stderr).expect("Not UTF-8");
            eprintln!("{}", sout);
            eprintln!("{}", serr);
            exit(1);
        }
    }
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = String::from("");
        match &self.command.args {
            Some(args) => fmt_args.push_str(&format!("{}", args)),
            None => fmt_args.push_str("None"),
        }
        write!(
            f,
            "  - Command: {}\n  - Command Args: {}\n",
            fmt_single_parameter(&self.command.name),
            fmt_args
        )
    }
}
