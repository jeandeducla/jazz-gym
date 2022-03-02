use std::str::FromStr;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod game;
mod notes;
mod player;
mod polysines;
mod sinewave;
mod source;

use game::{Command, Game};

use crate::notes::Interval;

fn main() {
    let mut game: Option<Game> = None;

    let mut rl = Editor::<()>::new();

    game::print_rules();

    let mut challenge_num = 0;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match game.as_mut() {
                Some(game) => {
                    match Interval::from_str(&line) {
                        Ok(interval) => {
                            game.challenges[challenge_num].answer(interval);
                            println!("Your answer was...");
                            match game.challenges[challenge_num].verify_user_answer() {
                                true => {
                                    println!("     > Correct!");
                                }
                                false => {
                                    println!("     x Uncorrect...");
                                    println!(
                                        "The correct answer was {:?}",
                                        game.challenges[challenge_num].correct_answer
                                    );
                                }
                            }
                            challenge_num += 1;
                            println!("Ok ! Let's go. Challenge number {}. Listen...", challenge_num);
                            game.challenges[challenge_num].play_correct_answer();
                            println!("What interval is it?");
                        }
                        Err(_) => println!("Invalid answer! (valid answers are : P1, m2, M2, m3 , M3, P4, P5, d5, m6, M6, m7, M7, P8")
                    }
                }
                None => match Command::from_str(&line) {
                    Ok(cmd) => match cmd {
                        Command::Start => {
                            game = Some(Game::new());
                            println!("Ok ! Let's go. First challenge. Listen...");
                            game.as_ref().unwrap().challenges[challenge_num].play_correct_answer();
                            println!("What interval is it?");
                        }
                        Command::Quit => {
                            println!("See You !");
                            break;
                        }
                    },
                    Err(_) => {
                        println!("Wrong command! (type: 'start' to start and 'quit' to");
                        println!("quit the game)");
                    }
                },
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
