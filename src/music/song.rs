use super::bar::Bar;
use super::rhythm::{Tempo, TimeSignature};

pub struct Song {
    tempo: Tempo,
    time_signature: TimeSignature,
    bars: Vec<Bar>,
}

impl Song {
    pub fn new(tempo: Tempo, time_signature: TimeSignature) -> Self {
        Song {
            tempo,
            time_signature,
            bars: vec![],
        }
    }

    pub fn push(&mut self, bar: Bar) {
        self.bars.push(bar);
    }

    pub fn new_bar(&mut self) -> &mut Bar {
        self.bars.push(Bar::new(self.time_signature.clone()));
        self.bars.last_mut().unwrap()
    }

    pub fn play(&self) {
        for compass in &self.bars {
            compass.play(&self.tempo);
        }
    }
}
