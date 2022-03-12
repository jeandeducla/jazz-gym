use rustyline::{error::ReadlineError, Editor};
use std::fmt::{self, Display};
use std::str::FromStr;

use super::parameters::Parameters;
use crate::gym::games::game::Game;

pub fn start(editor: &mut Editor<()>) -> Result<(), ReadlineError> {
    menu();
    let mut parameters = Parameters::default();
    loop {
        match editor.readline(">> ") {
            Ok(line) => match Command::from_str(&line) {
                Ok(cmd) => match cmd {
                    Command::Start => {
                        let mut game = Game::new(
                            parameters.num_challenges,
                            Some(parameters.base_note.clone()),
                            Some(parameters.intervals.clone().into_iter().collect()),
                        );
                        let _ = game.play(editor);
                    }
                    Command::Parameters => {
                        parameters.start(editor);
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
