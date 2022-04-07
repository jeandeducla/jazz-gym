use super::note::Note;
use super::rhythm::{Metric, Tempo, TimeSignature};
use crate::audio::Player;

use rodio::dynamic_mixer;
use rodio::Source;

// TODO: SMALLEST_METRIC can only be lower than a quarter as we assume bpm is in quarter
pub const SMALLEST_METRIC: Metric = Metric::Eighth;

// TODO: make mut iterators over time division (Half, whole....)
#[derive(Debug)]
pub struct Bar {
    time_signature: TimeSignature,
    beats: Vec<Option<Note>>,
}

impl Bar {
    pub fn new(time_signature: TimeSignature) -> Self {
        let division_level =
            (time_signature.0 as f32 / SMALLEST_METRIC.as_fraction_of_beat()) as usize;
        let mut beats = Vec::with_capacity(division_level);
        for _ in (0..division_level).into_iter() {
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
                let delay = SMALLEST_METRIC.duration(&tempo).saturating_mul(i as u32);
                println!("note delay {:?}", delay);
                let note = note.as_sine(&tempo).delay(delay);
                input.add(note);
            }
        }
        let _ = player.stream_handle.play_raw(output);
        let bar_duration = self.time_signature.bar_duration(&tempo);
        println!("coucou: {:?}", bar_duration);
        std::thread::sleep(bar_duration);
    }
}
