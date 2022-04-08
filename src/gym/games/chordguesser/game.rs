use super::challenge::Challenge;

use crate::gym::score::Score;
use crate::music::intervals::Interval;
use crate::music::pitches::Pitch;

use rand::Rng;
use rustyline::Editor;
use std::cell::RefCell;
use std::fmt::{self, Display};
use std::iter::repeat;
use std::str::FromStr;

const DEFAULT_CHALLENGE_NUM: usize = 5;
pub const MAX_CHALLENGE_NUM: usize = 20;

#[derive(Debug)]
pub struct Game {
    pub challenges: RefCell<Vec<Challenge>>,
    base_note: Pitch,
    intervals: Vec<Interval>,
}

impl Game {
    pub fn new(
        challenge_num: usize,
        base_note: Option<Pitch>,
        intervals: Option<Vec<Interval>>,
    ) -> Self {
        let challenge_num = if challenge_num == 0 {
            DEFAULT_CHALLENGE_NUM
        } else {
            challenge_num
        };

        let intervals = match intervals {
            Some(mut intervals) => {
                intervals.sort();
                intervals
            }
            None => (0..13)
                .into_iter()
                .map(|i| Interval::from(i))
                .collect::<Vec<Interval>>(),
        };

        let base_note: Pitch = match base_note {
            Some(base_note) => base_note,
            None => rand::thread_rng().gen_range(0, 13).into(),
        };

        Game {
            challenges: RefCell::new(
                (0..challenge_num)
                    .map(|_| {
                        let correct_answer = {
                            let idx = rand::thread_rng().gen_range(0, intervals.len());
                            intervals.get(idx).unwrap()
                        };
                        Challenge::new(base_note.clone(), correct_answer.clone())
                    })
                    .collect(),
            ),
            base_note,
            intervals,
        }
    }

    fn available_commands_menu(&self) {
        self.intervals
            .iter()
            .enumerate()
            .map(|(idx, s)| format!("{:>2}> {}", idx + 1, s))
            .for_each(|s| println!("| {:<10}{}", "", s));
        let replay = format!("{}> {:?}", Command::Replay.to_string(), Command::Replay);
        println!("| ");
        println!("| {:<10} {}", "", replay);
        println!("| ");
    }

    pub fn navigate(&mut self, editor: &mut Editor<()>) -> Result<(), ()> {
        println!("| Ok! Let's go!");
        println!("| ");
        println!("| {}", self);

        let mut score = Score::new(self.challenges.borrow().len());

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
                            if let Some(interval) = self.intervals.get((num - 1) as usize) {
                                challenge.answer(interval.clone());

                                println!("|");
                                println!(
                                    "|    Your answer, '{}',  was...",
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

                                score.update(idx, challenge.verify_user_answer());
                                println!("|");
                                println!("| {}", score);

                                break;
                            };
                        }
                    }
                    Err(_) => {
                        return Err(());
                    }
                }
            }
        }

        let (correct_answers, total_answered) = score.compute();
        println!("| ");
        println!("| Game Over");
        println!("| Your score is: {} / {}", correct_answers, total_answered);

        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("{:>10}Game\n", "".to_string());

        let intervals = self
            .intervals
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        s.push_str(&format!(
            "|>   challenges: [{}] ({})\n",
            repeat("-")
                .take(self.challenges.borrow().len())
                .collect::<String>(),
            self.challenges.borrow().len(),
        ));

        s.push_str(&format!("|>   base note: {}\n", self.base_note.to_string()));
        s.push_str(&format!("|>   intervals: {}", intervals));

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
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

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Replay => "r".to_string(),
        }
    }
}
