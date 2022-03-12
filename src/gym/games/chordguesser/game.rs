use crate::gym::challenge::Challenge;
use crate::gym::score::Score;
use crate::music::intervals::Interval;
use crate::music::notes::Note;

use rustyline::Editor;
use std::fmt::{self, Display};
use std::iter::repeat;
use std::str::FromStr;

const DEFAULT_CHALLENGE_NUM: usize = 5;
pub const MAX_CHALLENGE_NUM: usize = 20;

#[derive(Debug)]
pub struct Game {
    pub challenges: Vec<Challenge>,
    base_note: Option<Note>,
    intervals: Option<Vec<Interval>>,
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
            base_note,
            intervals,
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
        println!("| Ok! Let's go!");
        println!("| ");
        println!("| {}", self);

        let mut score = repeat('-')
            .take(self.challenges.len())
            .collect::<Vec<char>>();

        for (idx, challenge) in self.challenges.iter_mut().enumerate() {
            println!("| ");
            println!("| [{}] Listen...What interval is it? ", idx + 1);
            println!("|     (type 'replay' to replay the challenge)");
            challenge.play_correct_answer();

            loop {
                let readline = editor.readline("|___} ");

                match readline {
                    Ok(input) => {
                        if let Ok(_) = Command::from_str(&input) {
                            challenge.play_correct_answer();
                        } else if let Ok(interval) = Interval::from_str(&input) {
                            challenge.answer(interval);

                            println!("|    Your answer was...");
                            match challenge.verify_user_answer() {
                                true => {
                                    println!("|         > Correct!");
                                }
                                false => {
                                    println!("|         x Uncorrect...");
                                    println!(
                                        "|    The correct answer was {}",
                                        challenge.correct_answer
                                    );
                                }
                            }

                            if let Some(s) = score.get_mut(idx) {
                                *s = match challenge.verify_user_answer() {
                                    true => '*',
                                    false => '_',
                                };
                            };
                            println!("| [{}]", score.iter().collect::<String>());

                            break;
                        } else {
                            println!("| Invalid command: type 'replay' to replay the current challenge or type a valid interval");
                        }
                    }
                    Err(_) => {
                        return Err(());
                    }
                }
            }
        }

        println!("| ");
        println!("| Game Over");
        println!("| Your score is: {}", self.get_current_score());
        println!();

        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let base_note = match self.base_note.as_ref() {
            Some(note) => note.to_string(),
            None => "All".to_owned(),
        };

        let intervals = match self.intervals.as_ref() {
            Some(intervals) => {
                if intervals.is_empty() {
                    "All".to_owned()
                } else {
                    intervals
                        .iter()
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                }
            }
            None => "All".to_owned(),
        };

        let s = format!(
            "          Game\n|>   challenges: [{}] ({})\n|>   base_note: {}\n|>   intervals: {}",
            repeat("-").take(self.challenges.len()).collect::<String>(),
            self.challenges.len(),
            base_note,
            intervals,
        );
        write!(f, "{}", s)
    }
}

pub enum Command {
    Replay,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "replay" => Ok(Command::Replay),
            _ => Err(()),
        }
    }
}
// println!("You will hear a two-note interval and your goal is to ");
// println!("recognize it by typing one of the following:");
// println!(" - 'P1': for a Perfect Unission");
// println!(" - 'm2': for a Minor Second");
// println!(" - 'M2': for a Major Second");
// println!(" - 'm3': for a Minor Third");
// println!(" - 'M3': for a Major Third");
// println!(" - 'P4': for a Perfect Fourth");
// println!(" - 'P5': for a Perfect Fifth");
// println!(" - 'd5': for a Diminished Fifth");
// println!(" - 'm6': for a Minor Sixth ");
// println!(" - 'M6': for a Major Sixth");
// println!(" - 'm7': for a Minor Seventh");
// println!(" - 'M7': for a Major Seventh");
// println!(" - 'P8': for a Perfect Octave");
// println!("Do you want to start a new game? (type: 'start' to start");
// println!("and 'quit' to quit the game)");
