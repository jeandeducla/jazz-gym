use super::note::Note;
use super::rythm::{Tempo, TimeSignature};
use crate::audio::Player;

use rodio::Sink;

#[derive(Debug)]
pub struct Compass {
    tempo: Tempo,
    beats: Vec<Option<Note>>,
}

impl Compass {
    pub fn new(tempo: Tempo, time_signature: &TimeSignature) -> Self {
        let mut beats = Vec::with_capacity(time_signature.0 * 2);
        for _ in (0..time_signature.0 * 2).into_iter() {
            beats.push(None);
        }

        Compass { tempo, beats }
    }

    pub fn insert(&mut self, note: Note, beat_position: usize) {
        if let Some(notes) = self.beats.get_mut(beat_position) {
            *notes = Some(note);
        }
    }

    pub fn play(&self) {
        let player = Player::new();
        let mut beats: Vec<Sink> = Vec::new();
        for beat in &self.beats {
            println!("iouh");
            let sink = match beat {
                Some(note) => {
                    let sink = player.sink().unwrap();
                    sink.append(note.into_sine(&self.tempo));
                    // beats.push(sink)
                    sink
                }
                None => {
                    let sink = player.sink().unwrap();
                    let mut silence = Note::new(
                        crate::music::pitches::Pitch::C4,
                        crate::music::rythm::Duration::Eighth,
                    );
                    silence.amplify(0.0);
                    sink.append(silence.into_sine(&self.tempo));
                    // beats.push(sink)
                    sink
                }
            };
            player.play(&vec![sink]);
        }
        println!("{:?}", beats.len());
        // beats
    }
}
