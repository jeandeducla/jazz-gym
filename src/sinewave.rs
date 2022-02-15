const AMPLITUDE: f32 = 8192.0;

#[derive(Debug)]
pub struct SineWave {
    freq: f32,
    sample_rate: f32,
    num_points: usize,
    current_point: usize,
}

impl SineWave {
    pub fn new(freq: f32, sample_rate: f32, num_points: usize) -> Self {
        SineWave {
            freq,
            sample_rate,
            num_points,
            current_point: 0,
        }
    }

    pub fn num_points(&self) -> usize {
        self.num_points
    }
}

impl Iterator for SineWave {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_point >= self.num_points {
            return None;
        }
        let value =
            self.current_point as f32 * 2.0 * ::std::f32::consts::PI * self.freq / self.sample_rate;
        let value = (value.sin() * AMPLITUDE) as i16;
        self.current_point += 1;
        Some(value)
    }
}
