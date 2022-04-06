#[derive(Copy, Clone, Debug)]
pub struct TimeSignature(pub usize, pub usize);

impl TimeSignature {
    pub fn sub_beats_per_bar(&self) -> usize {
        self.0 * MAX_BEAT_DIVISION
    }

    pub fn compass_duration(&self, tempo: &Tempo) -> std::time::Duration {
        let duration: Duration = self.1.into();
        println!("PD {:?}", duration);
        let res = duration.seconds(tempo).saturating_mul(self.0 as u32);
        println!("RES {:?}", res);
        res
    }
}

const MAX_BEAT_DIVISION: usize = 2;

#[derive(Copy, Clone, Debug)]
pub enum Duration {
    Whole,
    Half,
    Quarter,
    Eighth,
}

impl Duration {
    pub fn seconds(&self, bpm: &Tempo) -> std::time::Duration {
        // TODO: could have some overflow situations here
        let min_beat_duration = bpm.min_time_division().as_millis() as u64;
        println!("min beat duration {}", min_beat_duration);
        match &self {
            Duration::Whole => std::time::Duration::from_millis(16 * min_beat_duration),
            Duration::Half => std::time::Duration::from_millis(4 * min_beat_duration),
            Duration::Quarter => std::time::Duration::from_millis(2 * min_beat_duration),
            Duration::Eighth => std::time::Duration::from_millis(min_beat_duration),
        }
    }
}

impl From<usize> for Duration {
    fn from(value: usize) -> Self {
        match value {
            2 => Duration::Half,
            4 => Duration::Quarter,
            8 => Duration::Eighth,
            _ => Duration::Eighth,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tempo(pub f32, pub Duration);

impl Tempo {
    pub fn new(bpm: f32, duration: Duration) -> Self {
        Tempo(bpm, duration)
    }

    pub fn min_time_division(&self) -> std::time::Duration {
        let milliseconds_per_beat = (60 * 1000) as f32 / (MAX_BEAT_DIVISION as f32 * self.0);
        println!("millis per beat: {}", milliseconds_per_beat);
        // let min_beat_millis = match &self.1 {
        //     Duration::Whole => milliseconds_per_beat / 8.,
        //     Duration::Half => milliseconds_per_beat / 4.,
        //     Duration::Quarter => milliseconds_per_beat / 2.,
        //     Duration::Eighth => milliseconds_per_beat,
        // };
        println!("min beat millis {}", milliseconds_per_beat);
        std::time::Duration::from_millis(milliseconds_per_beat as u64)
    }
}
