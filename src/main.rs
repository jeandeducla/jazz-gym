use std::str::FromStr;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod game;
mod music;
mod player;
mod source;

use crate::game::{Command, Game};
use crate::music::notes::Note;

fn main() {
    let mut session: Option<Game> = None;

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
                    Command::Start => {
                        session = Some(Game::new(2, Some(Note::C4), Some(vec![])));
                    }
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
}
