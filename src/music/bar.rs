use std::time::Duration;

use super::note::Note;
use super::rythm::{Tempo, TimeSignature};
use crate::audio::Player;

use rodio::dynamic_mixer;
use rodio::source::Source;
use rodio::source::Zero;

// TODO: make mut iterators over time division (Half, whole....)
// TODO: COMPASS is in Spanish lol, Bar is the actual correct name
#[derive(Debug)]
pub struct Bar {
    time_signature: TimeSignature,
    beats: Vec<Option<Note>>,
}

impl Bar {
    pub fn new(time_signature: TimeSignature) -> Self {
        let mut beats = Vec::with_capacity(time_signature.sub_beats_per_bar());
        for _ in (0..time_signature.sub_beats_per_bar()).into_iter() {
            beats.push(None);
        }
        Bar {
            beats,
            time_signature,
        }
    }

    // TODO: handle wrong index
    pub fn insert(&mut self, note: Note, beat_position: usize) {
        if let Some(notes) = self.beats.get_mut(beat_position) {
            *notes = Some(note);
        }
    }

    pub fn play(&self, tempo: &Tempo) {
        let player = Player::new();
        let (input, output) = dynamic_mixer::mixer(2, 44100);
        for (i, beat) in self.beats.iter().enumerate() {
            if let Some(note) = beat {
                let delay = tempo.min_time_division().saturating_mul(i as u32);
                println!("note delay {:?}", delay);
                let note = note
                    .as_sine(&tempo)
                    // .take_crossfade_with(Zero::<f32>::new(2, 44100), Duration::from_millis(350))
                    .delay(delay);
                input.add(note);
            }
        }
        let _ = player.stream_handle.play_raw(output);
        let bar_duration = self.time_signature.compass_duration(&tempo);
        println!("coucou: {:?}", bar_duration);
        std::thread::sleep(bar_duration);
        // std::thread::sleep(Duration::from_secs(10));
    }
}
