use std::process::{exit, Command};

use crate::config::Test;

pub fn test(test: Option<Test>) {
    match test {
        Some(config) => run(config),
        None => {
            println!("Error: No linting config provided");
            exit(1);
        }
    };
}

fn run(test: Test) {
    println!("Fisherman: Testing with {}", test.tester);
    let output = Command::new("which")
        .arg(&test.tester)
        .output()
        .expect("cannot run which");

    if !output.status.success() {
        println!("Fisherman Error: Tester `{}` not found.", &test.tester);
        exit(1);
    }

    let binding = Vec::new();
    let args: &Vec<String> = match &test.tester_args {
        Some(args) => args,
        None => &binding,
    };

    let output = Command::new(&test.tester).args(args).output().expect("");
    if !output.status.success() {
        let sout = String::from_utf8(output.stdout).expect("Not UTF-8");
        let serr = String::from_utf8(output.stderr).expect("Not UTF-8");
        println!("{}", sout);
        println!("{}", serr);
    }
    let success = output.status.success();

    if success {
        println!("Fisherman: Testing done.");
    } else {
        println!("Fisherman: Testing failed.");
        exit(1);
    }
}
