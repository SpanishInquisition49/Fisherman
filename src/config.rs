use colored::Colorize;
use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub lint: Option<Lint>,
    pub test: Option<Test>,
    // TODO: add test and commit message validation logic
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt: String = "".to_owned();
        match &self.lint {
            Some(lint) => fmt.push_str(&format!("{}{}", " Lint:\n".bold().green(), lint)),
            None => fmt.push_str("Lint: disabled\n"),
        };
        match &self.test {
            Some(test) => fmt.push_str(&format!("{}{}", "󰙨 Test:\n".bold().green(), test)),
            None => fmt.push_str("Test: disabled"),
        }
        write!(f, "{}", fmt)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lint {
    pub linter: String,
    pub file_ext: String,
    pub single_file: bool,
    pub linter_args: Option<Args>,
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = "".to_owned();
        match &self.linter_args {
            Some(args) => fmt_args.push_str(&format!("{}", args)),
            None => fmt_args.push_str("None"),
        }
        write!(
            f,
            " - Linter: {}\n - File Extension: {}\n - Run only on edited files: {}\n - Linter Args: {}\n",
            fmt_single_parameter(&self.linter), fmt_single_parameter(&self.file_ext), self.single_file.to_string().purple(), fmt_args
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub tester: String,
    pub tester_args: Option<Args>,
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = "".to_owned();
        match &self.tester_args {
            Some(args) => fmt_args.push_str(&format!("{}", args)),
            None => fmt_args.push_str("None"),
        }
        write!(
            f,
            " - Tester: {}\n - Tester Args: {}\n",
            fmt_single_parameter(&self.tester),
            fmt_args
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Args(pub Vec<String>);

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = "".to_owned();
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

fn fmt_single_parameter(parameter: &str) -> String {
    format!("{}{}{}", "\"".yellow(), parameter.yellow(), "\"".yellow())
}
