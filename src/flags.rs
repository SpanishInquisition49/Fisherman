use std::str::FromStr;

pub enum Flags {
    Init,
    Lint,
    Test,
    PreCommit,
    CommitMessage,
    ApplyHooks,
    InvalidFlag,
    Help,
    ShowConfig,
}

impl FromStr for Flags {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "-i" | "--init" => Ok(Flags::Init),
            "-l" | "--lint" => Ok(Flags::Lint),
            "-t" | "--test" => Ok(Flags::Test),
            "-c" | "--pre-commit" => Ok(Flags::PreCommit),
            "-m" | "--commit-message" => Ok(Flags::CommitMessage),
            "-a" | "--apply-hooks" => Ok(Flags::ApplyHooks),
            "-h" | "--help" => Ok(Flags::Help),
            "-s" | "--show-config" => Ok(Flags::ShowConfig),
            _ => Ok(Flags::InvalidFlag),
        }
    }
}

impl ToString for Flags {
    fn to_string(&self) -> String {
        let s = match *self {
            Flags::Init => "-i",
            Flags::Lint => "-l",
            Flags::Test => "-t",
            Flags::PreCommit => "-c",
            Flags::CommitMessage => "-m",
            Flags::ApplyHooks => "-a",
            Flags::InvalidFlag => "INVALID FLAGS",
            Flags::Help => "-h",
            Flags::ShowConfig => "-s",
        };
        String::from(s)
    }
}
