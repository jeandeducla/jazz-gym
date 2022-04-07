use rodio::source::SineWave;
use rodio::Sink;
use rodio::Source;
use std::time::Duration;

use crate::music::bar::Bar;
use crate::music::note::Note;
use crate::music::song::Song;
use crate::music::tempo::Tempo;
use crate::music::timesignature::TimeSignature;
use crate::{
    audio::Player,
    music::{intervals::Interval, metric, pitches::Pitch},
};

#[derive(Debug)]
pub struct Challenge {
    base_note: Pitch,
    pub correct_answer: Interval,
    pub user_answer: Option<Interval>,
}

impl Challenge {
    // TODO: possible to have referece instead of cloning?
    pub fn new(base_note: Pitch, correct_answer: Interval) -> Self {
        Challenge {
            base_note,
            correct_answer,
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
        let tempo = Tempo::new(180.0);
        let time_signature = TimeSignature(4);

        let mut song = Song::new(tempo.clone(), time_signature.clone());

        let mut bar = Bar::new(time_signature.clone());
        println!("{:?}", bar);
        bar.insert(Note::new(Pitch::C4, crate::music::metric::Metric::Whole), 0);
        bar.insert(
            Note::new(Pitch::E4, crate::music::metric::Metric::Quarter),
            2,
        );
        bar.insert(
            Note::new(Pitch::G4, crate::music::metric::Metric::Quarter),
            4,
        );
        bar.insert(
            Note::new(Pitch::B4, crate::music::metric::Metric::Quarter),
            6,
        );
        song.push(bar);

        let mut compass = Bar::new(time_signature.clone());
        compass.insert(
            Note::new(Pitch::B4, crate::music::metric::Metric::Quarter),
            0,
        );
        compass.insert(
            Note::new(Pitch::G4, crate::music::metric::Metric::Quarter),
            2,
        );
        compass.insert(
            Note::new(Pitch::E4, crate::music::metric::Metric::Quarter),
            4,
        );
        compass.insert(
            Note::new(Pitch::C4, crate::music::metric::Metric::Quarter),
            6,
        );
        song.push(compass);

        song.play();
    }

    pub fn play_correct_answer(&self) {
        self.play(&self.correct_answer);
    }
}
