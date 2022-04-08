use crate::music::note::Note;
use crate::music::rhythm::{Metric, Tempo, TimeSignature};
use crate::music::song::Song;
use crate::music::{intervals::Interval, pitches::Pitch};

#[derive(Debug)]
pub struct Challenge {
    base_note: Pitch,
    pub correct_answer: Interval,
    pub user_answer: Option<Interval>,
}

impl Challenge {
    // TODO: possible to have reference instead of cloning?
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
        let time_signature = TimeSignature(2);

        let mut song = Song::new(tempo.clone(), time_signature.clone());

        let bar = song.new_bar();
        bar.insert(Note::new(self.base_note.clone(), Metric::Quarter), 0);

        let second_note = self.base_note.add_interval(&answer);
        bar.insert(Note::new(second_note, Metric::Quarter), 2);

        song.play();
    }

    pub fn play_correct_answer(&self) {
        self.play(&self.correct_answer);
    }
}
