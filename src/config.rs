use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub lint: Option<Lint>,
    // TODO: add test and commit message validation logic
}

impl Config {
    pub fn to_string(&self) -> String {
        match &self.lint {
            Some(lint) => format!("Lint:\n{}", lint.to_string()),
            None => format!("Lint: disabled"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Lint {
    pub linter: String,
    pub file_ext: String,
    pub single_file: bool,
    pub linter_args: Option<Vec<String>>,
}

impl Lint {
    pub fn to_string(&self) -> String {
        format!(
            " - Linter: {}\n - File Extension: {}\n - Run only on edited files: {}\n - Linter Args: {:?}",
            self.linter, self.file_ext, self.single_file, self.linter_args
        )
    }
}
