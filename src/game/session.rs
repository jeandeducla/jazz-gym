use std::fmt::{self, Display};
use std::str::FromStr;

use rustyline::{error::ReadlineError, Editor};

use super::game::Game;
use super::games;
use crate::music::{intervals::Interval, notes::Note};

pub struct Session {
    game: Option<Game>,
}

impl Session {
    pub fn new() -> Self {
        Session { game: None }
    }

    pub fn start(&mut self, editor: &mut Editor<()>) -> Result<(), ReadlineError> {
        menu();
        loop {
            match editor.readline(">> ") {
                Ok(line) => match Command::from_str(&line) {
                    Ok(cmd) => match cmd {
                        Command::Games => {
                            let _ = games::start(editor);
                            menu();
                        }
                        Command::Quit => {
                            println!("See you mfk!");
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

    pub fn from_editor(editor: &mut Editor<()>) -> Result<Self, ReadlineError> {
        println!(". How many challenges do you want? (1..20)");
        let challenge_num = editor.readline(". ==> ")?;
        let challenge_num = usize::from_str(&challenge_num).unwrap_or(0);

        println!(". Do you want to pick a base note?");
        let base_note = editor.readline(". ==> ")?;
        let base_note = Note::from_str(&base_note).ok();

        println!(". Do you want to pick specific intervals?");
        let intervals = editor.readline(". ==> ")?;
        let intervals = Some(
            intervals
                .split(",")
                .filter_map(|s| Interval::from_str(s).ok())
                .collect::<Vec<Interval>>(),
        );

        Ok(Session {
            game: Some(Game::new(challenge_num, base_note, intervals)),
        })
    }

    pub fn take(&mut self) -> Option<Game> {
        self.game.take()
    }
}

pub enum Command {
    Games,
    Quit,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "1" => Ok(Command::Games),
            "0" => Ok(Command::Quit),
            _ => Err(()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Games => "games",
            Command::Quit => "quit",
        };
        write!(f, "{}", s)
    }
}

pub fn welcome() {
    println!("  OooOoo                         .oOOOo.                 ");
    println!("      O                         .O     o                 ");
    println!("      o                         o                        ");
    println!("      O                         O                        ");
    println!("      o  .oOoO' ooOO ooOO       O   .oOOo O   o `oOOoOO. ");
    println!("      O  O   o    o    o        o.      O o   O  O  o  o ");
    println!("O     o  o   O   O    O          O.    oO O   o  o  O  O ");
    println!("`OooOO'  `OoO'o OooO OooO         `OooO'  `OoOO  O  o  o ");
    println!("                                              o          ");
    println!("                                           OoO'          ");
    println!();
    println!();
    println!("Jazz Gym is a game to practice jazz from a terminal.");
    println!();
}

pub fn menu() {
    println!();
    println!(" Menu");
    println!();
    println!("  [1]> games ");
    println!();
    println!("  [0]< quit ");
    println!();
}
