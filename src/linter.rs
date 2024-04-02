use regex::bytes::Regex;
use std::{
    io::{self, Write},
    process::{exit, Command},
};

use crate::config::Lint;

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
    let staged_files = staged_files.split("\n");
    let re = Regex::new(&lint.file_ext).unwrap();
    for file in staged_files {
        if re.is_match(file.as_bytes()) {
            println!("{}", file);
            let binding = Vec::new();
            let args: &Vec<String> = match &lint.linter_args {
                Some(args) => &args,
                None => &binding,
            };

            let output = Command::new(&lint.linter)
                .args(args)
                .arg(file)
                .output()
                .expect("");
            if !output.status.success() {
                res = false;
                io::stdout().write_all(&output.stdout).unwrap();
                io::stderr().write_all(&output.stderr).unwrap();
            }
        }
    }
    res
}

fn lint_project(lint: &Lint) -> bool {
    println!("Fisherman: Run linter `{}` on whole project", &lint.linter);
    let binding = Vec::new();
    let args: &Vec<String> = match &lint.linter_args {
        Some(args) => &args,
        None => &binding,
    };

    let output = Command::new(&lint.linter).args(args).output().expect("");
    if !output.status.success() {
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }
    output.status.success()
}
