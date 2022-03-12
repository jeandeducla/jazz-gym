use rustyline::{error::ReadlineError, Editor};
use std::collections::HashSet;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::usize;

use super::game::MAX_CHALLENGE_NUM;
use crate::music::intervals::Interval;
use crate::music::notes::Note;

#[derive(Debug)]
pub struct Parameters {
    pub num_challenges: usize,
    pub base_note: Option<Note>,
    pub intervals: Option<HashSet<Interval>>,
}

impl Parameters {
    pub fn new() -> Self {
        Parameters::default()
    }

    fn set_num_challenges(&mut self, num_challenges: usize) -> Result<(), ()> {
        if num_challenges <= MAX_CHALLENGE_NUM {
            self.num_challenges = num_challenges;
            return Ok(());
        }
        Err(())
    }

    pub fn navigate(&mut self, editor: &mut Editor<()>) {
        loop {
            menu();
            match editor.readline(">> ") {
                Ok(line) => match Command::from_str(&line) {
                    Ok(cmd) => match cmd {
                        Command::NumChallenges => self.navigate_num_challenges(editor),
                        Command::BaseNote => self.navigate_base_note(editor),
                        Command::Intervals => self.navigate_intervals(editor),
                        Command::Reset => *self = Parameters::new(),
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

    fn navigate_num_challenges(&mut self, editor: &mut Editor<()>) {
        loop {
            self.num_challenges_menu();
            match editor.readline(">> ") {
                Ok(num) => {
                    if let Ok(num) = usize::from_str(&num) {
                        if num == 0 {
                            break;
                        }
                        if let Err(_) = self.set_num_challenges(num) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            };
        }
    }

    fn navigate_base_note(&mut self, editor: &mut Editor<()>) {
        loop {
            self.base_note_menu();
            match editor.readline(">> ") {
                Ok(num) => {
                    if let Ok(num) = u8::from_str(&num) {
                        if num == 0 {
                            break;
                        }
                        self.base_note = Some(Note::from(num - 1));
                    } else if num == "@" {
                        self.base_note = None;
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            };
        }
    }

    fn navigate_intervals(&mut self, editor: &mut Editor<()>) {
        loop {
            self.intervals_menu();
            match editor.readline(">> ") {
                Ok(num) => {
                    if let Ok(num) = u8::from_str(&num) {
                        if num == 0 {
                            break;
                        }
                        let interval = Interval::from(num - 1);
                        match &mut self.intervals {
                            Some(intervals) => {
                                if !intervals.remove(&interval) {
                                    intervals.insert(interval);
                                }
                            }
                            None => {
                                let mut intervals = HashSet::new();
                                intervals.insert(interval);
                                self.intervals = Some(intervals);
                            }
                        }
                    } else if num == "@" {
                        self.intervals.take();
                    } else {
                        break;
                    }
                }
                Err(_) => {
                    break;
                }
            };
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
                (0..13)
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
                        let checked = if let Some(base_note) = &self.base_note {
                            match idx == base_note.position() as usize {
                                true => "x",
                                false => ".",
                            }
                        } else {
                            "."
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

        let checked = if self.base_note.is_none() { "x" } else { "." };
        println!();
        println!("  [ @]({})> Random", checked);
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
                let checked = if let Some(intervals) = &self.intervals {
                    match intervals.get(&interval).is_some() {
                        true => "x",
                        false => ".",
                    }
                } else {
                    "."
                };
                format!(
                    "{:<10}",
                    format!("  [{:>2}]({})> {}", idx + 1, checked, interval)
                )
            })
            .for_each(|s| println!("{}", s));

        let checked = if self.intervals.is_none() { "x" } else { "." };
        println!();
        println!("  [ @]({})> Random", checked);
        println!();
        println!("  [0]< back");
        println!();
    }
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            num_challenges: 5,
            base_note: Some(Note::C4),
            intervals: None,
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
