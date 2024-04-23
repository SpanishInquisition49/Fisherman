use crate::config::{Args, Test};
use inquire::Text;
use std::process::{exit, Command};

pub fn test_init() -> Test {
    let mut test_config = Test {
        tester: "".to_owned(),
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
        Some(args) => &args.0,
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
