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
            Some(lint) => fmt.push_str(&format!("Lint:\n{}", lint)),
            None => fmt.push_str("Lint: disabled"),
        };
        match &self.test {
            Some(test) => fmt.push_str(&format!("Test:\n{}", test)),
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
        write!(
            f,
            " - Linter: \"{}\"\n - File Extension: \"{}\"\n - Run only on edited files: {}\n - Linter Args: {:?}\n",
            self.linter, self.file_ext, self.single_file, self.linter_args
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
        write!(
            f,
            " - Tester: \"{}\"\n - Tester Args: {:?}\n",
            self.tester, self.tester_args
        )
    }
}
