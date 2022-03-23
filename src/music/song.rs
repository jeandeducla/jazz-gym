use super::compass::Compass;
use super::rythm::{Tempo, TimeSignature};
use crate::audio::Player;

pub struct Song {
    tempo: Tempo,
    time_signature: TimeSignature,
    compasses: Vec<Compass>,
}

impl Song {
    pub fn new(tempo: Tempo, time_signature: TimeSignature) -> Self {
        Song {
            tempo,
            time_signature,
            compasses: vec![],
        }
    }

    pub fn push(&mut self, compass: Compass) {
        self.compasses.push(compass);
    }

    pub fn play(&self) {
        // let player = Player::new();
        for compass in &self.compasses {
            println!("coucou");
            compass.play();
            // player.play(&v);
            println!("coucou");
        }
    }
}
