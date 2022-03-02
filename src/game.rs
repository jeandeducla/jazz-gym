use crate::{
    notes::{Interval, Note},
    player::Player,
    polysines::PolySines,
    sinewave::SineWave,
};

use rand::Rng;
use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub challenges: Vec<Challenge>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            challenges: (1..10).map(|_| Challenge::new()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Challenge {
    base_note: Note,
    pub correct_answer: Interval,
    user_answer: Option<Interval>,
}

impl Challenge {
    fn new() -> Self {
        Challenge {
            base_note: rand::thread_rng().gen_range(0, 13).into(),
            correct_answer: rand::thread_rng().gen_range(0, 13).into(),
            user_answer: None,
        }
    }

    pub fn answer(&mut self, answer: Interval) {
        self.user_answer = Some(answer);
    }

    pub fn verify(&self, answer: &Interval) -> bool {
        self.correct_answer == *answer
    }

    pub fn verify_user_answer(&self) -> bool {
        self.verify(self.user_answer.as_ref().unwrap())
    }

    fn play(&self, answer: &Interval) {
        let sample_rate = 44100;
        let num_points = 44100 * 2;
        let player = Player::new(sample_rate);
        let base_note = SineWave::new(self.base_note.freqency(), 44100.0, num_points);
        let second_note = SineWave::new(
            (self.base_note.add_interval(answer)).freqency(),
            44100.0,
            num_points,
        );
        let src = PolySines::new(base_note, second_note);
        player.play(&src.collect::<Vec<i16>>());
    }

    pub fn play_correct_answer(&self) {
        self.play(&self.correct_answer);
    }
}

pub enum Command {
    Start,
    Quit,
    Replay,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "start" => Ok(Command::Start),
            "quit" => Ok(Command::Quit),
            "r" => Ok(Command::Replay),
            _ => Err(()),
        }
    }
}
