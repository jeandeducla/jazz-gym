use rustyline::{error::ReadlineError, Editor};
use std::str::FromStr;

use super::game::Game;
use crate::music::{intervals::Interval, notes::Note};

pub struct Session {
    game: Option<Game>,
}

impl Session {
    pub fn new() -> Self {
        Session { game: None }
    }

    pub fn from_editor(editor: &mut Editor<()>) -> Result<Self, ReadlineError> {
        println!(". How many challenges do you want? (1..20)");
        let challenge_num = editor.readline(". ==> ")?;
        let challenge_num = usize::from_str(&challenge_num).unwrap_or(0);

        println!(". Do you want to pick a base note?");
        let base_note = editor.readline(". ==> ")?;
        let base_note = Note::from_str(&base_note).ok();

        println!(". Do you want to pick specific intervals?");
        let intervals = editor.readline(". ==> ")?;
        let intervals = Some(
            intervals
                .split(",")
                .filter_map(|s| Interval::from_str(s).ok())
                .collect::<Vec<Interval>>(),
        );

        Ok(Session {
            game: Some(Game::new(challenge_num, base_note, intervals)),
        })
    }

    pub fn take(&mut self) -> Option<Game> {
        self.game.take()
    }
}
