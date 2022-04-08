use rustyline::Editor;
use std::fmt::{self, Display};
use std::str::FromStr;
use std::usize;

use super::game::MAX_CHALLENGE_NUM;

#[derive(Debug)]
pub struct Parameters {
    pub num_challenges: usize,
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
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters { num_challenges: 5 }
    }
}

enum Command {
    Reset,
    NumChallenges,
    Back,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "0" => Ok(Command::Back),
            "1" => Ok(Command::NumChallenges),
            "2" => Ok(Command::Reset),
            _ => Err(()),
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Command::Back => "back",
            Command::NumChallenges => "number of challenges",
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
    println!("  [2]> {} ", Command::Reset);
    println!();
    println!("  [0]< {} ", Command::Back);
    println!();
}
