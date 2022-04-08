use crate::music::note::Note;
use crate::music::pitches::Pitch;
use crate::music::rhythm::{Metric, Tempo, TimeSignature};
use crate::music::song::Song;

#[derive(Debug)]
pub struct Challenge {
    first_note: Pitch,
    second_note: Pitch,
    pub correct_answer: bool,
    pub user_answer: Option<bool>,
}

impl Challenge {
    pub fn new(base_note: Pitch, second_note: Pitch) -> Self {
        Challenge {
            first_note: base_note,
            second_note,
            correct_answer: base_note < second_note,
            user_answer: None,
        }
    }

    pub fn answer(&mut self, answer: bool) {
        self.user_answer = Some(answer);
    }

    pub fn verify(&self, answer: &bool) -> bool {
        self.correct_answer == *answer
    }

    pub fn verify_user_answer(&self) -> bool {
        self.verify(self.user_answer.as_ref().unwrap())
    }

    fn play(&self) {
        let tempo = Tempo::new(120.0);
        let time_signature = TimeSignature(2);

        let mut song = Song::new(tempo.clone(), time_signature.clone());
        let bar = song.new_bar();
        bar.insert(Note::new(self.first_note.clone(), Metric::Quarter), 0);
        bar.insert(Note::new(self.second_note, Metric::Quarter), 2);

        song.play();
    }

    pub fn play_correct_answer(&self) {
        self.play();
    }
}
