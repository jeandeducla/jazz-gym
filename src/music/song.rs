use super::bar::Bar;
use super::rhythm::{Tempo, TimeSignature};

pub struct Song {
    tempo: Tempo,
    time_signature: TimeSignature,
    compasses: Vec<Bar>,
}

impl Song {
    pub fn new(tempo: Tempo, time_signature: TimeSignature) -> Self {
        Song {
            tempo,
            time_signature,
            compasses: vec![],
        }
    }

    pub fn push(&mut self, compass: Bar) {
        self.compasses.push(compass);
    }

    pub fn play(&self) {
        for compass in &self.compasses {
            compass.play(&self.tempo);
        }
    }
}
