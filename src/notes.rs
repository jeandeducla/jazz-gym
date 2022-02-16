pub enum Notes {
    C4,
    Db4,
    D4,
    Eb4,
    E4,
    F4,
    Gb4,
    G4,
    Ab4,
    A4,
    Bb4,
    B4,
    C5,
}

impl Notes {
    pub fn freqency(&self) -> f32 {
        match self {
            Notes::C4 => 261.3,
            Notes::Db4 => 277.18,
            Notes::D4 => 293.66,
            Notes::Eb4 => 313.13,
            Notes::E4 => 329.63,
            Notes::F4 => 349.23,
            Notes::Gb4 => 369.99,
            Notes::G4 => 392.0,
            Notes::Ab4 => 414.30,
            Notes::A4 => 440.0,
            Notes::Bb4 => 466.16,
            Notes::B4 => 493.88,
            Notes::C5 => 523.25,
        }
    }
}
