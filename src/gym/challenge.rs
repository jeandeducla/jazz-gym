use rodio::source::SineWave;
use rodio::Sink;
use rodio::Source;
use std::time::Duration;

use crate::music::compass;
use crate::music::compass::Compass;
use crate::music::note::Note;
use crate::music::rythm::{Tempo, TimeSignature};
use crate::music::song::Song;
use crate::{
    audio::Player,
    music::{intervals::Interval, pitches::Pitch},
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
        let tempo = Tempo::new(120.0);
        let time_signature = TimeSignature(4, 4);

        let mut song = Song::new(tempo.clone(), time_signature.clone());
        let mut compass = Compass::new(tempo, &time_signature);

        compass.insert(Note::new(Pitch::C4, crate::music::rythm::Duration::Half), 0);
        compass.insert(Note::new(Pitch::E4, crate::music::rythm::Duration::Half), 4);

        song.push(compass);
        song.play();
        // let player = Player::new();

        // let base_note = SineWave::new(self.base_note.freqency())
        //     .take_duration(Duration::from_secs(3))
        //     .amplify(0.2);
        // let sink_1 = Sink::try_new(&player.stream_handle).unwrap();
        // sink_1.append(base_note);

        // let second_note = SineWave::new((self.base_note.add_interval(answer)).freqency())
        //     .take_duration(Duration::from_secs(4))
        //     .amplify(0.2);
        // let sink_2 = Sink::try_new(&player.stream_handle).unwrap();
        // sink_2.append(second_note);

        // player.play(&vec![sink_1, sink_2]);
    }

    pub fn play_correct_answer(&self) {
        self.play(&self.correct_answer);
    }
}
