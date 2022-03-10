use std::str::FromStr;

use rustyline::Editor;
use rustyline::{error::ReadlineError, Result};

mod game;
mod music;
mod player;
mod source;

use crate::game::{Command, Session};

fn main() -> Result<()> {
    let mut session = Session::new();

    let mut rl = Editor::<()>::new();

    // TODO: rules are going to depend on the challenges that you choose
    game::print_rules();

    loop {
        if let Some(mut game) = session.take() {
            let _ = game.play(&mut rl);
        }

        println!("Type 'start' to start a new game and 'quit' to quit Jazz Gym");
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match Command::from_str(&line) {
                Ok(cmd) => match cmd {
                    Command::Start => session = Session::from_editor(&mut rl)?,
                    Command::Quit => {
                        println!("See You!");
                        break;
                    }
                    _ => {
                        println!("Wrong command! (type: 'start' to start game and 'quit' to quit Jazz Gym)");
                    }
                },
                Err(_) => {
                    println!("Wrong command! (type: 'start' to start a new game and 'quit' to quit Jazz Gym)");
                }
            },
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
