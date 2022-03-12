use rustyline::{error::ReadlineError, Editor};
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::usize;

use crate::music::intervals::Interval;
use crate::music::notes::Note;

#[derive(Debug)]
pub struct Parameters {
    pub num_challenges: usize,
    pub base_note: Note,
    pub intervals: HashSet<Interval>,
}

impl Parameters {
    // TODO: Back button
    // TODO: input validation
    pub fn navigate(&mut self, editor: &mut Editor<()>) {
        loop {
            menu();
            match editor.readline(">> ") {
                Ok(line) => match Command::from_str(&line) {
                    Ok(cmd) => match cmd {
                        Command::NumChallenges => loop {
                            self.num_challenges_menu();
                            match editor.readline(">> ") {
                                Ok(num) => {
                                    if let Ok(num) = usize::from_str(&num) {
                                        if num == 0 {
                                            break;
                                        }
                                        self.num_challenges = num;
                                    } else {
                                        break;
                                    }
                                }
                                Err(_) => {
                                    break;
                                }
                            };
                        },
                        Command::BaseNote => loop {
                            self.base_note_menu();
                            match editor.readline(">> ") {
                                Ok(num) => {
                                    if let Ok(num) = u8::from_str(&num) {
                                        if num == 0 {
                                            break;
                                        }
                                        self.base_note = Note::from(num - 1);
                                    } else {
                                        break;
                                    }
                                }
                                Err(_) => {
                                    break;
                                }
                            };
                        },
                        Command::Intervals => loop {
                            self.intervals_menu();
                            match editor.readline(">> ") {
                                Ok(num) => {
                                    if let Ok(num) = u8::from_str(&num) {
                                        if num == 0 {
                                            break;
                                        }
                                        let interval = Interval::from(num - 1);
                                        if !self.intervals.remove(&interval) {
                                            self.intervals.insert(interval);
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                Err(_) => {
                                    break;
                                }
                            };
                        },
                        Command::Reset => {
                            // self = Parameters::default();
                            println!("Parameters reset");
                            menu();
                        }
                        Command::Back => return,
                    },
                    Err(_) => {
                        println!("Wrong command! Type either 1 or 2");
                        menu();
                    }
                },
                Err(_) => break,
            }
        }
    }

    fn num_challenges_menu(&self) {
        println!();
        println!(" Number of Challenges");
        println!();
        (0..5)
            .map(|i| {
                (0..20)
                    .into_iter()
                    .filter(|c| c % 4 == i)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>()
            .into_iter()
            .filter(|v| !v.is_empty())
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|idx| {
                        let checked = match idx + 1 == self.num_challenges {
                            true => "x",
                            false => ".",
                        };
                        format!("  [{}]> {:>2}", checked, idx + 1)
                    })
                    .collect::<Vec<String>>()
                    .join("  ")
            })
            .for_each(|s| println!("{}", s));
        println!();
        println!("  [0]< back");
        println!();
    }

    fn base_note_menu(&self) {
        println!();
        println!(" Base Note");
        println!();
        (0..4)
            .map(|i| {
                (0..12)
                    .into_iter()
                    .filter(|c| c % 3 == i)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>()
            .into_iter()
            .filter(|v| !v.is_empty())
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|idx| {
                        let checked = match idx == self.base_note.position() as usize {
                            true => "x",
                            false => ".",
                        };
                        let note: Note = (idx as u8).into();
                        format!(
                            "{:<15}",
                            format!("  [{:>2}]({})> {}", idx + 1, checked, note)
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .for_each(|s| println!("{}", s));
        println!();
        println!("  [0]< back");
        println!();
    }

    fn intervals_menu(&self) {
        println!();
        println!(" Intervals");
        println!();
        (0..13)
            .into_iter()
            .map(|idx| {
                let interval: Interval = (idx as u8).into();
                let checked = match self.intervals.get(&interval).is_some() {
                    true => "x",
                    false => ".",
                };
                format!(
                    "{:<10}",
                    format!("  [{:>2}]({})> {}", idx + 1, checked, interval)
                )
            })
            .for_each(|s| println!("{}", s));
        println!();
        println!("  [0]< back");
        println!();
    }
}

impl Default for Parameters {
    fn default() -> Self {
        // TODO: hashet default?
        Parameters {
            num_challenges: 5,
            base_note: Note::C4,
            intervals: HashSet::new(),
        }
    }
}

enum Command {
    Reset,
    NumChallenges,
    BaseNote,
    Intervals,
    Back,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "0" => Ok(Command::Back),
            "1" => Ok(Command::NumChallenges),
            "2" => Ok(Command::BaseNote),
            "3" => Ok(Command::Intervals),
            "4" => Ok(Command::Reset),
            _ => Err(()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Back => "back",
            Command::NumChallenges => "number of challenges",
            Command::BaseNote => "base note",
            Command::Intervals => "intervals",
            Command::Reset => "reset",
        };
        write!(f, "{}", s)
    }
}

pub fn menu() {
    println!();
    println!(" Parameters");
    println!();
    println!("  [1]> {} ", Command::NumChallenges);
    println!("  [2]> {} ", Command::BaseNote);
    println!("  [3]> {} ", Command::Intervals);
    println!("  [4]> {} ", Command::Reset);
    println!();
    println!("  [0]< {} ", Command::Back);
    println!();
}
