use super::tempo::Tempo;

#[derive(Copy, Clone, Debug)]
pub enum Metric {
    Whole,
    Half,
    Quarter,
    Eighth,
}

impl Metric {
    pub fn duration(&self, bpm: &Tempo) -> std::time::Duration {
        // TODO: could have some overflow situations here
        let beat_duration = bpm.beat_duration();
        match &self {
            Metric::Whole => beat_duration.checked_mul(4).unwrap(),
            Metric::Half => beat_duration.checked_mul(2).unwrap(),
            Metric::Quarter => beat_duration,
            Metric::Eighth => beat_duration.checked_div(2).unwrap(),
        }
    }

    pub fn as_fraction_of_beat(&self) -> f32 {
        match &self {
            Metric::Whole => 4.0,
            Metric::Half => 2.0,
            Metric::Quarter => 1.0,
            Metric::Eighth => 0.5,
        }
    }
}
