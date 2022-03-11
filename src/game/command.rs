use std::str::FromStr;

pub enum Command {
    Start,
    Quit,
    Replay,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "start" => Ok(Command::Start),
            "quit" => Ok(Command::Quit),
            "replay" => Ok(Command::Replay),
            _ => Err(()),
        }
    }
}
