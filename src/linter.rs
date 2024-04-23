use inquire::{Confirm, Text};
use regex::bytes::Regex;
use std::process::{exit, Command};

use crate::config::{Args, Lint};

pub fn lint_init() -> Lint {
    let mut lint_config = Lint {
        linter: "".to_owned(),
        file_ext: "".to_owned(),
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

pub fn lint(lint: Option<Lint>) {
    match lint {
        Some(config) => run(config),
        None => {
            println!("Error: No linting config provided");
            exit(1);
        }
    };
}

fn run(lint: Lint) {
    let output = Command::new("which")
        .arg(&lint.linter)
        .output()
        .expect("cannot run which");

    if !output.status.success() {
        println!("Fisherman Error: Linter `{}` not found.", &lint.linter);
        exit(1);
    }
    let success = if lint.single_file {
        lint_file(&lint)
    } else {
        lint_project(&lint)
    };

    if success {
        println!("Fisherman: Linting done.");
    } else {
        println!("Fisherman: Linting failed.");
        exit(1);
    }
}

fn lint_file(lint: &Lint) -> bool {
    let mut res = true;
    println!("Fisherman: Run linter `{}` on staged files", &lint.linter);
    let staged_files = Command::new("git")
        .arg("diff")
        .arg("--cached")
        .arg("--name-only")
        .output()
        .expect("git cannot be executed");
    let staged_files = String::from_utf8(staged_files.stdout).unwrap();
    let staged_files = staged_files.split('\n');
    let re = Regex::new(&lint.file_ext).unwrap();
    for file in staged_files {
        if re.is_match(file.as_bytes()) {
            println!("{}", file);
            let binding = Vec::new();
            let args: &Vec<String> = match &lint.linter_args {
                Some(args) => &args.0,
                None => &binding,
            };

            let output = Command::new(&lint.linter)
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

fn lint_project(lint: &Lint) -> bool {
    println!("Fisherman: Run linter `{}` on whole project", &lint.linter);
    let binding = Vec::new();
    let args: &Vec<String> = match &lint.linter_args {
        Some(args) => &args.0,
        None => &binding,
    };

    let output = Command::new(&lint.linter).args(args).output().expect("");
    if !output.status.success() {
        let sout = String::from_utf8(output.stdout).expect("Not UTF-8");
        let serr = String::from_utf8(output.stderr).expect("Not UTF-8");
        println!("{}", sout);
        println!("{}", serr);
    }
    output.status.success()
}
