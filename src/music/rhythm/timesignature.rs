use super::tempo::Tempo;

// TODO: let's assume the lower number of the time signature is 4 (i.e. quarter notes)
#[derive(Copy, Clone, Debug)]
pub struct TimeSignature(pub usize);

impl TimeSignature {
    pub fn bar_duration(&self, tempo: &Tempo) -> std::time::Duration {
        tempo.beat_duration().checked_mul(self.0 as u32).unwrap()
    }
}
