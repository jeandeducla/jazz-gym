use rustyline::{error::ReadlineError, Editor};
use std::fmt::{self, Display};
use std::str::FromStr;

use super::chordguesser;

pub fn start(editor: &mut Editor<()>) -> Result<(), ReadlineError> {
    menu();
    loop {
        match editor.readline(">> ") {
            Ok(line) => match Command::from_str(&line) {
                Ok(cmd) => match cmd {
                    Command::ChordGuesser => {
                        let _ = chordguesser::start(editor);
                        menu();
                    }
                    Command::Back => {
                        break;
                    }
                },
                Err(_) => {
                    println!("Wrong command! Type either 1 or 2");
                    menu();
                }
            },
            Err(_) => break,
        }
    }

    Ok(())
}

enum Command {
    ChordGuesser,
    Back,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "0" => Ok(Command::Back),
            "1" => Ok(Command::ChordGuesser),
            _ => Err(()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Back => "back",
            Command::ChordGuesser => "chord guesser",
        };
        write!(f, "{}", s)
    }
}

fn menu() {
    println!();
    println!(" Games");
    println!();
    println!("  [1]> {} ", Command::ChordGuesser);
    println!();
    println!("  [0]< {} ", Command::Back);
    println!();
}
