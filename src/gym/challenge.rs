use crate::{
    music::{intervals::Interval, notes::Note},
    player::Player,
    source::PolySines,
    source::SineWave,
};

use rand::Rng;

#[derive(Debug)]
pub struct Challenge {
    base_note: Note,
    pub correct_answer: Interval,
    pub user_answer: Option<Interval>,
}

impl Challenge {
    // TODO: possible to have referece instead of cloning?
    pub fn new(base_note: Option<Note>, intervals: Option<Vec<Interval>>) -> Self {
        let base_note: Note = match base_note {
            Some(base_note) => base_note,
            None => rand::thread_rng().gen_range(0, 13).into(),
        };

        let correct_answer: Interval = match intervals {
            Some(intervals) => {
                if !intervals.is_empty() {
                    let idx = rand::thread_rng().gen_range(0, intervals.len());
                    intervals.get(idx).unwrap().to_owned()
                } else {
                    rand::thread_rng().gen_range(0, 13).into()
                }
            }
            None => rand::thread_rng().gen_range(0, 13).into(),
        };

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
