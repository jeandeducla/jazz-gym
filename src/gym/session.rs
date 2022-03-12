use std::fmt::{self, Display};
use std::str::FromStr;

use rustyline::{error::ReadlineError, Editor};

use crate::gym::games::games;

pub struct Session {}

impl Session {
    pub fn new() -> Self {
        Session {}
    }

    pub fn start(&mut self, editor: &mut Editor<()>) -> Result<(), ReadlineError> {
        menu();
        loop {
            match editor.readline(">> ") {
                Ok(line) => match Command::from_str(&line) {
                    Ok(cmd) => match cmd {
                        Command::Games => {
                            let _ = games::navigate(editor);
                            menu();
                        }
                        Command::Quit => {
                            println!("See you!");
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
