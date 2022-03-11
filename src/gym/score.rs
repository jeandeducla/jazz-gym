use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Score {
    pub correct_answers: usize,
    pub total_answers: usize,
}

impl Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} / {}", self.correct_answers, self.total_answers)
    }
}
