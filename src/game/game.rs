use super::challenge::Challenge;
use super::score::Score;
use crate::music::intervals::Interval;
use crate::music::notes::Note;

const DEFAULT_CHALLENGE_NUM: usize = 5;

#[derive(Debug)]
pub struct Game {
    pub challenges: Vec<Challenge>,
}

impl Game {
    pub fn new(
        challenge_num: usize,
        base_note: Option<Note>,
        intervals: Option<Vec<Interval>>,
    ) -> Self {
        let challenge_num = if challenge_num == 0 {
            DEFAULT_CHALLENGE_NUM
        } else {
            challenge_num
        };

        Game {
            challenges: (0..challenge_num)
                .map(|_| Challenge::new(base_note.clone(), intervals.clone()))
                .collect(),
        }
    }

    pub fn get_current_score(&self) -> Score {
        Score {
            correct_answers: self
                .challenges
                .iter()
                .filter(|c| c.user_answer.is_some())
                .fold(
                    0,
                    |acc, c| {
                        if c.verify_user_answer() {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                ),
            total_answers: self.challenges.len(),
        }
    }
}
