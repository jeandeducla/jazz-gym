use crate::gym::challenge::Challenge;
use crate::gym::score::Score;
use crate::music::intervals::Interval;
use crate::music::notes::Note;

use rustyline::Editor;
use std::cell::RefCell;
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::iter::repeat;
use std::str::FromStr;

const DEFAULT_CHALLENGE_NUM: usize = 5;
pub const MAX_CHALLENGE_NUM: usize = 20;

#[derive(Debug)]
pub struct Game {
    pub challenges: RefCell<Vec<Challenge>>,
    base_note: Option<Note>,
    intervals: Option<HashSet<Interval>>,
}

impl Game {
    pub fn new(
        challenge_num: usize,
        base_note: Option<Note>,
        intervals: Option<HashSet<Interval>>,
    ) -> Self {
        let challenge_num = if challenge_num == 0 {
            DEFAULT_CHALLENGE_NUM
        } else {
            challenge_num
        };

        Game {
            challenges: RefCell::new(
                (0..challenge_num)
                    .map(|_| Challenge::new(base_note.clone(), intervals.clone()))
                    .collect(),
            ),
            base_note,
            intervals,
        }
    }

    pub fn get_current_score(&self) -> Score {
        Score {
            correct_answers: self
                .challenges
                .borrow()
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
            total_answers: self.challenges.borrow().len(),
        }
    }

    fn available_commands_menu(&self) {
        match self.intervals.as_ref() {
            Some(intervals) => intervals
                .into_iter()
                .collect::<Vec<&Interval>>()
                .iter()
                .enumerate()
                .map(|(idx, s)| format!("{:<10}", format!("|         {:>2}> {}", idx + 1, s)))
                .for_each(|s| println!("{}", s)),
            None => (0..13)
                .into_iter()
                .map(|i| Interval::from(i))
                .enumerate()
                .map(|(idx, s)| format!("{:<10}", format!("|         {:>2}> {}", idx + 1, s)))
                .for_each(|s| println!("{}", s)),
        }
        println!("| ");
        println!("|         r> Replay");
        println!("| ");
    }

    pub fn navigate(&mut self, editor: &mut Editor<()>) -> Result<(), ()> {
        println!("| Ok! Let's go!");
        println!("| ");
        println!("| {}", self);

        let mut score = repeat('-')
            .take(self.challenges.borrow().len())
            .collect::<Vec<char>>();

        for (idx, challenge) in self.challenges.borrow_mut().iter_mut().enumerate() {
            println!("| ");
            println!("| [{}] Listen...What interval is it? ", idx + 1);
            println!("| ");
            challenge.play_correct_answer();
            self.available_commands_menu();

            loop {
                let readline = editor.readline("|___} ");

                match readline {
                    Ok(input) => {
                        if let Ok(_) = Command::from_str(&input) {
                            challenge.play_correct_answer();
                        } else if let Ok(num) = u8::from_str(&input) {
                            let interval = match self
                                .intervals
                                .as_ref()
                                .unwrap()
                                .into_iter()
                                .collect::<Vec<&Interval>>()
                                .get(num as usize)
                            {
                                Some(interval) => interval.clone(),
                                None => break,
                            };
                            challenge.answer(interval.clone());

                            println!("|");
                            println!(
                                "|    Your answer {},  was...",
                                challenge.user_answer.as_ref().unwrap()
                            );
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

        let s =
            format!(
            "          Game\n|>   challenges: [{}] ({})\n|>   base_note: {}\n|>   intervals: {}",
            repeat("-").take(self.challenges.borrow().len()).collect::<String>(),
            self.challenges.borrow().len(),
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
            "r" => Ok(Command::Replay),
            _ => Err(()),
        }
    }
}
