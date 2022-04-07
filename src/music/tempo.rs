/// We assume that the Tempo bpm is in number of quarter notes per minute
#[derive(Clone, Debug)]
pub struct Tempo(pub f32);

impl Tempo {
    pub fn new(bpm: f32) -> Self {
        Tempo(bpm)
    }

    pub fn beat_duration(&self) -> std::time::Duration {
        let milliseconds_per_beat = (60 * 1000) as f32 / self.0;
        std::time::Duration::from_millis(milliseconds_per_beat as u64)
    }
}
