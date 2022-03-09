use std::str::FromStr;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod game;
mod music;
mod player;
mod source;

use crate::game::{Command, Game};
use crate::music::intervals::Interval;
use crate::music::notes::Note;

fn main() {
    let mut session: Option<Game> = None;

    let mut rl = Editor::<()>::new();

    game::print_rules();

    let mut challenge_num = 0;
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match session.as_mut() {
                Some(game) => {
                    if let Ok(replay) = Command::from_str(&line) {
                        if let Command::Replay = replay {
                            if let Some(challenge) = game.challenges.get_mut(challenge_num) {
                                challenge.play_correct_answer();
                            }
                        }
                    } else {
                        match Interval::from_str(&line) {
                            Ok(interval) => {
                                if let Some(challenge) = game.challenges.get_mut(challenge_num) {
                                    challenge.answer(interval);
                                    println!("Your answer was...");
                                    match challenge.verify_user_answer() {
                                        true => {
                                            println!("     > Correct!");
                                        }
                                        false => {
                                            println!("     x Uncorrect...");
                                            println!(
                                                "The correct answer was {}",
                                                challenge.correct_answer
                                            );
                                        }
                                    }
                                }
                                println!("Your current score is: {}\n", game.get_current_score());
                                challenge_num += 1;
                                if let Some(challenge) = game.challenges.get(challenge_num) {
                                    println!("Challenge number {}. Listen...", challenge_num + 1);
                                    challenge.play_correct_answer();
                                    println!("What interval is it? (type 'replay' to replay the interval)");
                                } else {
                                    println!("Game is over!");
                                    println!("Your final score is: {}", game.get_current_score());
                                    session = None;
                                    challenge_num = 0;
                                }

                            }
                            Err(_) => println!("Invalid answer! (valid answers are : P1, m2, M2, m3 , M3, P4, P5, d5, m6, M6, m7, M7, P8")
                        }
                    }
                }
                None => match Command::from_str(&line) {
                    Ok(cmd) => match cmd {
                        Command::Start => {
                            println!("Ok ! Let's go");
                            session = Some(Game::new(0, Some(Note::C4), Some(vec![])));
                            println!("Challenge number {}. Listen...", challenge_num + 1);
                            session.as_ref().unwrap().challenges[challenge_num]
                                .play_correct_answer();
                            println!("What interval is it? (type 'replay' to replay the interval)");
                        }
                        Command::Quit => {
                            println!("See You !");
                            break;
                        }
                        _ => {
                            println!("Wrong command! (type: 'start' to start and 'quit' to");
                            println!("quit the game)");
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
