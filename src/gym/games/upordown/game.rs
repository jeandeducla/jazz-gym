use super::challenge::Challenge;

use crate::gym::score::Score;
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
}

impl Game {
    pub fn new(challenge_num: usize) -> Self {
        let challenge_num = if challenge_num == 0 {
            DEFAULT_CHALLENGE_NUM
        } else {
            challenge_num
        };

        Game {
            challenges: RefCell::new(
                (0..challenge_num)
                    .map(|_| {
                        let first_note: Pitch = rand::thread_rng().gen_range(0, 13).into();
                        let mut second_note: Pitch = rand::thread_rng().gen_range(0, 13).into();
                        while second_note == first_note {
                            second_note = rand::thread_rng().gen_range(0, 13).into();
                        }
                        Challenge::new(first_note, second_note)
                    })
                    .collect(),
            ),
        }
    }

    fn available_commands_menu(&self) {
        let up = format!("{}> {:?}", Command::Up.to_string(), Command::Up);
        let down = format!("{}> {:?}", Command::Down.to_string(), Command::Down);
        let replay = format!("{}> {:?}", Command::Replay.to_string(), Command::Replay);
        println!("| {:<10}{}", "", up);
        println!("| {:<10}{}", "", down);
        println!("| ");
        println!("| {:<10}{}", "", replay);
        println!("| ");
    }

    pub fn navigate(&mut self, editor: &mut Editor<()>) -> Result<(), ()> {
        println!("| Ok! Let's go!");
        println!("| ");
        println!("| {}", self);

        let mut score = Score::new(self.challenges.borrow().len());

        for (idx, challenge) in self.challenges.borrow_mut().iter_mut().enumerate() {
            println!("| ");
            println!(
                "| [{}] Listen...Is the second note higher or lower? ",
                idx + 1
            );
            println!("| ");
            challenge.play_correct_answer();
            self.available_commands_menu();

            loop {
                let readline = editor.readline("|___} ");

                match readline {
                    Ok(input) => {
                        if let Ok(cmd) = Command::from_str(&input) {
                            if let Command::Replay = cmd {
                                challenge.play_correct_answer();
                            } else {
                                let answer = match cmd {
                                    Command::Up => true,
                                    Command::Down => false,
                                    _ => unreachable!(),
                                };
                                challenge.answer(answer);

                                println!("|");
                                println!(
                                    "|    Your answer, '{}',  was...",
                                    match challenge.user_answer.as_ref().unwrap() {
                                        true => "higher",
                                        false => "lower",
                                    }
                                );
                                match challenge.verify_user_answer() {
                                    true => {
                                        println!("|         > Correct!");
                                    }
                                    false => {
                                        println!("|         x Uncorrect...");
                                        println!(
                                            "|    The correct answer was {}",
                                            match challenge.correct_answer {
                                                true => "higher",
                                                false => "lower",
                                            }
                                        );
                                    }
                                }

                                score.update(idx, challenge.verify_user_answer());
                                println!("|");
                                println!("| {}", score);

                                break;
                            }
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
        println!();

        Ok(())
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = format!("{:>10}Game\n", "".to_string());
        s.push_str(&format!(
            "|>   challenges: [{}] ({})",
            repeat("-")
                .take(self.challenges.borrow().len())
                .collect::<String>(),
            self.challenges.borrow().len(),
        ));
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Command {
    Replay,
    Up,
    Down,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "r" => Ok(Command::Replay),
            "1" => Ok(Command::Up),
            "2" => Ok(Command::Down),
            _ => Err(()),
        }
    }
}

impl ToString for Command {
    fn to_string(&self) -> String {
        match self {
            Command::Replay => "r".to_string(),
            Command::Up => "1".to_string(),
            Command::Down => "2".to_string(),
        }
    }
}
