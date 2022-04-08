use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Score {
    challenges: Vec<Option<bool>>,
}

impl Score {
    pub fn new(questions_num: usize) -> Self {
        Score {
            challenges: (0..questions_num).map(|_| None).collect(),
        }
    }

    pub fn update(&mut self, idx: usize, answer: bool) {
        if let Some(s) = self.challenges.get_mut(idx) {
            *s = Some(answer);
        };
    }

    pub fn compute(&self) -> (usize, usize) {
        (
            self.challenges
                .iter()
                .filter_map(|c| *c)
                .map(|b| b as usize)
                .sum(),
            self.challenges.iter().filter_map(|c| *c).count(),
        )
    }
}

impl Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.challenges
                .iter()
                .map(|res| {
                    match res {
                        Some(res) => {
                            if *res {
                                '*'
                            } else {
                                '-'
                            }
                        }
                        None => '_',
                    }
                })
                .collect::<String>()
        )
    }
}
