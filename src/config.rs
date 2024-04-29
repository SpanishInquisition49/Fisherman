use colored::Colorize;
use core::fmt;
use inquire::{Confirm, MultiSelect, Text};
use serde::{Deserialize, Serialize};
use std::{env::current_dir, fs::{self, File}, io::Write, os::unix::fs::PermissionsExt, path::Path, process::Command};

use crate::{commit_message::CommitMessage, flags::Flags, logger::log_error, pre_commit::PreCommit};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub root_directory: String,
    pub pre_commit: Option<PreCommit>,
    pub commit_message: Option<CommitMessage>,
}

impl Config {
    pub fn init() -> Config {
        let mut config: Config = Config {
            root_directory: String::from(""),
            pre_commit: None,
            commit_message: None,
        };
        config.root_directory = String::from(current_dir().unwrap().to_str().unwrap());
        let config_options = vec!["Pre Commit", "Commit Message"];
        let answers = match MultiSelect::new("Select the hooks to enable:", config_options).prompt()
        {
            Ok(res) => res,
            Err(e) => {
                log_error(&e.to_string(), true);
                unreachable!();
            }
        };
        for option in answers {
            match option {
                "Pre Commit" => config.pre_commit = Some(PreCommit::init()),
                "Commit Message" => config.commit_message = Some(CommitMessage::init()),
                _ => (),
            }
        }
        config
    }
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt: String = String::from("");
        match &self.pre_commit {
            Some(pre_commit) => {
                fmt.push_str(&format!("[{}]\n{}", "Pre Commit".bold().blue(), pre_commit))
            }
            None => (),
        };
        match &self.commit_message {
            Some(commit_message) => fmt.push_str(&format!(
                "[{}]\n{}",
                "Commit Message".bold().blue(),
                commit_message
            )),
            None => (),
        }
        write!(f, "{}", fmt)
    }
}

impl Config {
    pub fn apply(&self) {
        if self.pre_commit.is_some() {
            let code = format!("#!/bin/bash\ncd {}\nfisherman {} \"$1\"\n", &self.root_directory, Flags::PreCommit.to_string());
            generate_git_hook("pre-commit", &code);
        }
        if self.commit_message.is_some() {
            let code = format!(
                "#!/bin/bash\ncd {}\nnew_message=$(fisherman {} $1)\nif [[ $? != 0 ]]; then\nexit 1\nfi\necho $new_message > $1\nexit 0",
                &self.root_directory, Flags::CommitMessage.to_string()
            );
            generate_git_hook("commit-msg", &code);
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Args(pub Vec<String>);

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_args = String::from("");
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cmd {
    pub name: String,
    pub args: Option<Args>,
}

impl Cmd {
    pub fn init(command_prompt: &str) -> Cmd {
        let mut command = Cmd {
            name: String::from(""),
            args: None,
        };
        command.name = Text::new(command_prompt).prompt().unwrap();
        let args = Text::new("Program args:")
            .with_help_message("<esc> to skip")
            .prompt_skippable()
            .unwrap();
        command.args =
            args.map(|args| Args(args.split_whitespace().map(|v| v.to_string()).collect()));
        command
    }

    pub fn check(&self) -> bool {
        Command::new("which")
            .arg(&self.name)
            .output()
            .expect("cannot run which")
            .status
            .success()
    }
}

fn generate_git_hook(filename: &str, code: &str) {
    let path = format!("./.git/hooks/{}", filename);

    if Path::new(&path).exists() && !Confirm::new(&format!("File '{}' already extist, would you like to everwrite?", filename)).with_help_message("y/n").prompt().unwrap() {
        return;
    }
    let mut hook = File::create(&path).unwrap();
    let metadata = hook.metadata().unwrap();
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o751); // Execute/Read/write for owner and read for others.
    let _ = fs::set_permissions(path, permissions);
    match hook.write_all(code.as_bytes()) {
        Ok(_) => eprintln!("Hook '{}' created", filename),
        Err(e) => log_error(&format!("Could not create Hook '{}': {}", filename, e), true)
    }
}

pub fn fmt_single_parameter(parameter: &str) -> String {
    format!("{}{}{}", "\"".yellow(), parameter.yellow(), "\"".yellow())
}
