use rustyline::{error::ReadlineError, Editor};
use std::fmt::{self, Display};
use std::str::FromStr;

use crate::music::intervals::Interval;

use super::game::Game;
use super::parameters::Parameters;

pub fn navigate(editor: &mut Editor<()>) -> Result<(), ReadlineError> {
    menu();
    let mut parameters = Parameters::new();
    loop {
        match editor.readline(">> ") {
            Ok(line) => match Command::from_str(&line) {
                Ok(cmd) => match cmd {
                    Command::Start => {
                        let intervals = parameters
                            .intervals
                            .clone()
                            .map(|i| i.into_iter().collect::<Vec<Interval>>());

                        let mut game = Game::new(
                            parameters.num_challenges,
                            parameters.base_note.clone(),
                            intervals,
                        );
                        let _ = game.navigate(editor);
                    }
                    Command::Parameters => {
                        parameters.navigate(editor);
                        menu()
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
    Start,
    Parameters,
    Back,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "0" => Ok(Command::Back),
            "1" => Ok(Command::Start),
            "2" => Ok(Command::Parameters),
            _ => Err(()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Back => "back",
            Command::Start => "start",
            Command::Parameters => "parameters",
        };
        write!(f, "{}", s)
    }
}

fn menu() {
    println!();
    println!(" Chord Guesser");
    println!();
    println!("  [1]> {} ", Command::Start);
    println!("  [2]> {} ", Command::Parameters);
    println!();
    println!("  [0]< {} ", Command::Back);
    println!();
}
