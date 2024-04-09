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
            Some(lint) => fmt.push_str(&format!("{}{}", "Lint:\n".bold().green(), lint)),
            None => fmt.push_str("Lint: disabled"),
        };
        match &self.test {
            Some(test) => fmt.push_str(&format!("{}{}", "Test:\n".bold().green(), test)),
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
    pub linter_args: Option<Vec<String>>,
}

impl fmt::Display for Lint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = "".to_owned();
        match &self.linter_args {
            Some(args) => fmt_args.push_str(&format!(
                "{}{}{}",
                "[".red(),
                args.join(", ").yellow(),
                "]".red()
            )),
            None => fmt_args.push_str("None"),
        }
        write!(
            f,
            " - Linter: \"{}\"\n - File Extension: \"{}\"\n - Run only on edited files: {}\n - Linter Args: {}\n",
            self.linter.yellow(), self.file_ext.yellow(), self.single_file.to_string().purple(), fmt_args
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Test {
    pub tester: String,
    pub tester_args: Option<Vec<String>>,
}

impl fmt::Display for Test {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = "".to_owned();
        match &self.tester_args {
            Some(args) => {
                fmt_args.push_str("[");
                let mut index = 0;
                for arg in args {
                    if index == args.len() - 1 {
                        fmt_args.push_str(&format!(r#""{}""#, arg))
                    } else {
                        fmt_args.push_str(&format!(r#""{}", "#, arg));
                    }
                    index = index + 1;
                }
                fmt_args.push_str("]");
            }
            None => fmt_args.push_str("None"),
        }
        write!(
            f,
            " - Tester: \"{}\"\n - Tester Args: {}\n",
            self.tester, fmt_args
        )
    }
}
