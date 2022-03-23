#[derive(Clone, Copy)]
pub struct TimeSignature(pub usize, pub usize);

#[derive(Clone, Debug)]
pub enum Duration {
    Whole,
    Half,
    Quarter,
    Eighth,
}

impl Duration {
    pub fn time_duration(&self, bpm: &Tempo) -> std::time::Duration {
        // TODO: could have some overflow situations here
        let min_beat_duration = bpm.min_beat_duration().as_millis() as u64;
        match &self {
            Duration::Whole => std::time::Duration::from_millis(8 * min_beat_duration),
            Duration::Half => std::time::Duration::from_millis(4 * min_beat_duration),
            Duration::Quarter => std::time::Duration::from_millis(2 * min_beat_duration),
            Duration::Eighth => std::time::Duration::from_millis(min_beat_duration),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tempo(pub f32, pub Duration);

impl Default for Tempo {
    fn default() -> Self {
        Tempo(120., Duration::Half)
    }
}

impl Tempo {
    pub fn new(bpm: f32) -> Self {
        Tempo(bpm, Duration::Half)
    }

    fn set_beat_duration(&mut self, duration: Duration) {
        self.1 = duration;
    }

    fn min_beat_duration(&self) -> std::time::Duration {
        let milliseconds_per_beat = (60 * 1000) as f32 / self.0;
        let min_beat_millis = match &self.1 {
            Duration::Whole => milliseconds_per_beat / 8.,
            Duration::Half => milliseconds_per_beat / 4.,
            Duration::Quarter => milliseconds_per_beat / 2.,
            Duration::Eighth => milliseconds_per_beat,
        };

        std::time::Duration::from_millis(min_beat_millis as u64)
    }
}
