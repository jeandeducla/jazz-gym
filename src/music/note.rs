use rodio::source::SineWave;
use rodio::source::{Amplify, TakeDuration};
use rodio::Source;

use super::pitches::Pitch;
use super::rythm::{self, Tempo};

#[derive(Debug)]
pub struct Note {
    pitch: Pitch,
    duration: rythm::Duration,
    amplitude: f32,
}

impl Note {
    pub fn new(pitch: Pitch, duration: rythm::Duration) -> Self {
        Note {
            pitch,
            duration,
            amplitude: 0.2,
        }
    }

    pub fn as_sine(&self, tempo: &Tempo) -> Amplify<TakeDuration<SineWave>> {
        SineWave::new(self.pitch.freqency())
            .take_duration(self.duration.seconds(tempo))
            .amplify(self.amplitude)
    }
}
