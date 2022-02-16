use crate::source::Source;

const AMPLITUDE: f32 = 8192.0;

#[derive(Debug)]
pub struct SineWave {
    freq: f32,
    sample_rate: f32,
    target_num_samples: usize,
    num_samples: usize,
}

impl SineWave {
    pub fn new(freq: f32, sample_rate: f32, num_points: usize) -> Self {
        SineWave {
            freq,
            sample_rate,
            target_num_samples: num_points,
            num_samples: 0,
        }
    }

    fn value(&self, t: usize) -> f32 {
        (t as f32 * 2.0 * ::std::f32::consts::PI * self.freq / self.sample_rate).sin() * AMPLITUDE
    }
}

impl Source for SineWave {}

impl Iterator for SineWave {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_samples >= self.target_num_samples {
            return None;
        }
        let value = self.value(self.num_samples);
        self.num_samples += 1;
        Some(value as i16)
    }
}
