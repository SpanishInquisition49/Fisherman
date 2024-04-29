use colored::Colorize;
use core::fmt;
use inquire::MultiSelect;

use serde::{Deserialize, Serialize};

use crate::{linter::Lint, logger::log_error, tester::Test};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreCommit {
    pub lint: Option<Lint>,
    pub test: Option<Test>,
}

impl PreCommit {
    pub fn init() -> PreCommit {
        let mut pre_commit = PreCommit {
            lint: None,
            test: None,
        };
        let pre_commit_options = vec!["Linting", "Testing"];
        let answers =
            match MultiSelect::new("Select the feature to enable:", pre_commit_options).prompt() {
                Ok(res) => res,
                Err(e) => {
                    log_error(&e.to_string(), true);
                    unreachable!();
                }
            };
        for feature in answers {
            match feature {
                "Linting" => {
                    pre_commit.lint = Some(Lint::init());
                }
                "Testing" => {
                    pre_commit.test = Some(Test::init());
                }
                _ => (),
            };
        }
        pre_commit
    }

    pub fn run(&self) {
        if let Some(test) = &self.test {
            test.run();
        }
        if let Some(lint) = &self.lint {
            lint.run();
        }
    }
}

impl fmt::Display for PreCommit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt: String = String::from("");
        match &self.lint {
            Some(lint) => fmt.push_str(&format!(" {}{}", "Lint:\n".bold().green(), lint)),
            None => fmt.push_str(" Lint: disabled\n"),
        };
        match &self.test {
            Some(test) => fmt.push_str(&format!(" {}{}", "Test:\n".bold().green(), test)),
            None => fmt.push_str(" Test: disabled\n"),
        }
        write!(f, "{}", fmt)
    }
}
