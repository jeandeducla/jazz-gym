use rustyline::Editor;
use rustyline::Result;

mod audio;
mod gym;
mod music;

use crate::gym::{welcome, Session};
use crate::music::{
    compass::Compass,
    note::Note,
    pitches::Pitch,
    rythm::{Tempo, TimeSignature},
    song::Song,
};

fn main() -> Result<()> {
    let tempo = Tempo::new(90.0);
    let time_signature = TimeSignature(4, 4);

    let mut song = Song::new(tempo.clone(), time_signature.clone());
    let mut compass = Compass::new(tempo, &time_signature);
    println!("{:?}", compass);

    compass.insert(Note::new(Pitch::C4, crate::music::rythm::Duration::Half), 0);
    compass.insert(
        Note::new(Pitch::Eb4, crate::music::rythm::Duration::Whole),
        3,
    );
    compass.insert(Note::new(Pitch::E4, crate::music::rythm::Duration::Half), 4);
    println!("{:?}", compass);

    song.push(compass);
    song.play();
    // let mut rl = Editor::<()>::new();
    // welcome();
    // let mut session = Session::new();
    // let _ = session.start(&mut rl);
    Ok(())
}
