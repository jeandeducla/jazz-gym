use rodio::source::SineWave;
use rodio::Sink;
use rodio::Source;
use std::time::Duration;

use crate::{
    audio::Player,
    music::{intervals::Interval, notes::Note},
};

#[derive(Debug)]
pub struct Challenge {
    base_note: Note,
    pub correct_answer: Interval,
    pub user_answer: Option<Interval>,
}

impl Challenge {
    // TODO: possible to have referece instead of cloning?
    pub fn new(base_note: Note, correct_answer: Interval) -> Self {
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
        let player = Player::new();

        let base_note = SineWave::new(self.base_note.freqency())
            .take_duration(Duration::from_secs(3))
            .amplify(0.2);
        let sink_1 = Sink::try_new(&player.stream_handle).unwrap();
        sink_1.append(base_note);

        let second_note = SineWave::new((self.base_note.add_interval(answer)).freqency())
            .take_duration(Duration::from_secs(3))
            .amplify(0.2);
        let sink_2 = Sink::try_new(&player.stream_handle).unwrap();
        sink_2.append(second_note);

        player.play(&vec![sink_1, sink_2]);
    }

    pub fn play_correct_answer(&self) {
        self.play(&self.correct_answer);
    }
}
