use super::challenge::Challenge;
use super::score::Score;

#[derive(Debug)]
pub struct Game {
    pub challenges: Vec<Challenge>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            challenges: (0..3).map(|_| Challenge::new()).collect(),
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
