use super::challenge::Challenge;
use super::command::Command;
use super::score::Score;
use crate::music::intervals::Interval;
use crate::music::notes::Note;

use rustyline::Editor;
use std::str::FromStr;

const DEFAULT_CHALLENGE_NUM: usize = 5;

#[derive(Debug)]
pub struct Game {
    pub challenges: Vec<Challenge>,
}

impl Game {
    pub fn new(
        challenge_num: usize,
        base_note: Option<Note>,
        intervals: Option<Vec<Interval>>,
    ) -> Self {
        let challenge_num = if challenge_num == 0 {
            DEFAULT_CHALLENGE_NUM
        } else {
            challenge_num
        };

        Game {
            challenges: (0..challenge_num)
                .map(|_| Challenge::new(base_note.clone(), intervals.clone()))
                .collect(),
        }
    }

    pub fn get_current_score(&self) -> Score {
        Score {
            correct_answers: self
                .challenges
                .iter()
                .filter(|c| c.user_answer.is_some())
                .fold(
                    0,
                    |acc, c| {
                        if c.verify_user_answer() {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                ),
            total_answers: self.challenges.len(),
        }
    }

    pub fn play(&mut self, editor: &mut Editor<()>) -> Result<(), ()> {
        println!("\nOk ! Let's go");

        for (idx, challenge) in self.challenges.iter_mut().enumerate() {
            println!("\n[.] Challenge number {}", idx + 1);
            challenge.play_correct_answer();
            println!("What is the interval?");

            loop {
                let readline = editor.readline(" --> ");

                match readline {
                    Ok(input) => {
                        if let Ok(cmd) = Command::from_str(&input) {
                            if let Command::Replay = cmd {
                                challenge.play_correct_answer();
                            }
                        } else if let Ok(interval) = Interval::from_str(&input) {
                            challenge.answer(interval);

                            println!("Your answer was...");
                            match challenge.verify_user_answer() {
                                true => {
                                    println!("     > Correct!");
                                }
                                false => {
                                    println!("     x Uncorrect...");
                                    println!("The correct answer was {}", challenge.correct_answer);
                                }
                            }
                            break;
                        } else {
                            println!("Invalid command: type 'replay' to replay the current challenge or type a valid interval");
                        }
                    }
                    Err(_) => {
                        return Err(());
                    }
                }
            }
        }

        println!("\nGame Over");
        println!("Your score is: {}\n", self.get_current_score());

        Ok(())
    }
}
