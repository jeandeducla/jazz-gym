pub enum Notes {
    C4,
    D4,
    E4,
    F4,
    G4,
    A4,
    B4,
    C5,
}

impl Notes {
    pub fn freqency(&self) -> f32 {
        match self {
            Notes::C4 => 261.3,
            Notes::D4 => 293.66,
            Notes::E4 => 329.63,
            Notes::F4 => 349.23,
            Notes::G4 => 392.0,
            Notes::A4 => 440.0,
            Notes::B4 => 493.88,
            Notes::C5 => 523.25,
        }
    }
}
