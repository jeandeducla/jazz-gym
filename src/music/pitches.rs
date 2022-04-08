use super::intervals::Interval;

use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pitch {
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

impl Display for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Pitch::C4 => "C4",
            Pitch::Db4 => "Db4",
            Pitch::D4 => "D4",
            Pitch::Eb4 => "Eb4",
            Pitch::E4 => "E4",
            Pitch::F4 => "F4",
            Pitch::Gb4 => "Gb4",
            Pitch::G4 => "G4",
            Pitch::Ab4 => "Ab4",
            Pitch::A4 => "A4",
            Pitch::Bb4 => "Bb4",
            Pitch::B4 => "B4",
            Pitch::C5 => "C5",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Pitch {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "C4" => Ok(Pitch::C4),
            "Db4" => Ok(Pitch::Db4),
            "D4" => Ok(Pitch::D4),
            "Eb4" => Ok(Pitch::Eb4),
            "E4" => Ok(Pitch::E4),
            "F4" => Ok(Pitch::F4),
            "Gb4" => Ok(Pitch::Gb4),
            "G4" => Ok(Pitch::G4),
            "Ab4" => Ok(Pitch::Ab4),
            "A4" => Ok(Pitch::A4),
            "Bb4" => Ok(Pitch::Bb4),
            "B4" => Ok(Pitch::B4),
            "C5" => Ok(Pitch::C5),
            _ => Err(()),
        }
    }
}

impl Pitch {
    pub fn freqency(&self) -> f32 {
        match self {
            Pitch::C4 => 261.3,
            Pitch::Db4 => 277.18,
            Pitch::D4 => 293.66,
            Pitch::Eb4 => 313.13,
            Pitch::E4 => 329.63,
            Pitch::F4 => 349.23,
            Pitch::Gb4 => 369.99,
            Pitch::G4 => 392.0,
            Pitch::Ab4 => 414.30,
            Pitch::A4 => 440.0,
            Pitch::Bb4 => 466.16,
            Pitch::B4 => 493.88,
            Pitch::C5 => 523.25,
        }
    }

    pub fn position(&self) -> u8 {
        match self {
            Pitch::C4 => 0,
            Pitch::Db4 => 1,
            Pitch::D4 => 2,
            Pitch::Eb4 => 3,
            Pitch::E4 => 4,
            Pitch::F4 => 5,
            Pitch::Gb4 => 6,
            Pitch::G4 => 7,
            Pitch::Ab4 => 8,
            Pitch::A4 => 9,
            Pitch::Bb4 => 10,
            Pitch::B4 => 11,
            Pitch::C5 => 12,
        }
    }

    pub fn add_interval(&self, interval: &Interval) -> Pitch {
        (self.position() + interval.as_u8()).into()
    }
}

impl From<u8> for Pitch {
    fn from(value: u8) -> Self {
        match value {
            0 => Pitch::C4,
            1 => Pitch::Db4,
            2 => Pitch::D4,
            3 => Pitch::Eb4,
            4 => Pitch::E4,
            5 => Pitch::F4,
            6 => Pitch::Gb4,
            7 => Pitch::G4,
            8 => Pitch::Ab4,
            9 => Pitch::A4,
            10 => Pitch::Bb4,
            11 => Pitch::B4,
            12 => Pitch::C5,
            _ => Pitch::C5,
        }
    }
}

impl Into<u8> for Pitch {
    fn into(self) -> u8 {
        match self {
            Pitch::C4 => 0,
            Pitch::Db4 => 1,
            Pitch::D4 => 2,
            Pitch::Eb4 => 3,
            Pitch::E4 => 4,
            Pitch::F4 => 5,
            Pitch::Gb4 => 6,
            Pitch::G4 => 7,
            Pitch::Ab4 => 8,
            Pitch::A4 => 9,
            Pitch::Bb4 => 10,
            Pitch::B4 => 11,
            Pitch::C5 => 12,
        }
    }
}

// impl Add<Interval> for Note {
//     type Output = Self;
//
//     fn add(self, interval: Interval) -> Self::Output {
//         let note_position: u8 = self.into();
//         let interval: u8 = interval.into();
//         let new_note: u8 = note_position + interval;
//         new_note.into()
//     }
// }
