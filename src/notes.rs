use std::{ops::Add, str::FromStr};

#[derive(Debug)]
pub enum Note {
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

impl Note {
    pub fn freqency(&self) -> f32 {
        match self {
            Note::C4 => 261.3,
            Note::Db4 => 277.18,
            Note::D4 => 293.66,
            Note::Eb4 => 313.13,
            Note::E4 => 329.63,
            Note::F4 => 349.23,
            Note::Gb4 => 369.99,
            Note::G4 => 392.0,
            Note::Ab4 => 414.30,
            Note::A4 => 440.0,
            Note::Bb4 => 466.16,
            Note::B4 => 493.88,
            Note::C5 => 523.25,
        }
    }

    fn position(&self) -> u8 {
        match self {
            Note::C4 => 0,
            Note::Db4 => 1,
            Note::D4 => 2,
            Note::Eb4 => 3,
            Note::E4 => 4,
            Note::F4 => 5,
            Note::Gb4 => 6,
            Note::G4 => 7,
            Note::Ab4 => 8,
            Note::A4 => 9,
            Note::Bb4 => 10,
            Note::B4 => 11,
            Note::C5 => 12,
        }
    }

    pub fn add_interval(&self, interval: &Interval) -> Note {
        (self.position() + interval.as_u8()).into()
    }
}

impl From<u8> for Note {
    fn from(value: u8) -> Self {
        match value {
            0 => Note::C4,
            1 => Note::Db4,
            2 => Note::D4,
            3 => Note::Eb4,
            4 => Note::E4,
            5 => Note::F4,
            6 => Note::Gb4,
            7 => Note::G4,
            8 => Note::Ab4,
            9 => Note::A4,
            10 => Note::Bb4,
            11 => Note::B4,
            12 => Note::C5,
            _ => Note::C5,
        }
    }
}

impl Into<u8> for Note {
    fn into(self) -> u8 {
        match self {
            Note::C4 => 0,
            Note::Db4 => 1,
            Note::D4 => 2,
            Note::Eb4 => 3,
            Note::E4 => 4,
            Note::F4 => 5,
            Note::Gb4 => 6,
            Note::G4 => 7,
            Note::Ab4 => 8,
            Note::A4 => 9,
            Note::Bb4 => 10,
            Note::B4 => 11,
            Note::C5 => 12,
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

#[derive(Debug, PartialEq)]
pub enum Interval {
    PerfectUnission,
    MinorSecond,
    MajorSecond,
    MinorThird,
    MajorThird,
    PerfectFourth,
    DiminishedFifth,
    PerfectFifth,
    MinorSixth,
    MajorSixth,
    MinorSeventh,
    MajorSeventh,
    PerfectOctave,
}

impl Interval {
    pub fn as_u8(&self) -> u8 {
        match self {
            Interval::PerfectUnission => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::PerfectOctave => 12,
        }
    }
}

impl FromStr for Interval {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "P1" => Ok(Interval::PerfectUnission),
            "m2" => Ok(Interval::MinorSecond),
            "M2" => Ok(Interval::MajorSecond),
            "m3" => Ok(Interval::MinorThird),
            "M3" => Ok(Interval::MajorThird),
            "P4" => Ok(Interval::PerfectFourth),
            "P5" => Ok(Interval::PerfectFifth),
            "d5" => Ok(Interval::DiminishedFifth),
            "m6" => Ok(Interval::MinorSixth),
            "M6" => Ok(Interval::MajorSixth),
            "m7" => Ok(Interval::MinorSeventh),
            "M7" => Ok(Interval::MajorSeventh),
            "P8" => Ok(Interval::PerfectOctave),
            _ => Err(()),
        }
    }
}

impl Into<u8> for Interval {
    fn into(self) -> u8 {
        match self {
            Interval::PerfectUnission => 0,
            Interval::MinorSecond => 1,
            Interval::MajorSecond => 2,
            Interval::MinorThird => 3,
            Interval::MajorThird => 4,
            Interval::PerfectFourth => 5,
            Interval::DiminishedFifth => 6,
            Interval::PerfectFifth => 7,
            Interval::MinorSixth => 8,
            Interval::MajorSixth => 9,
            Interval::MinorSeventh => 10,
            Interval::MajorSeventh => 11,
            Interval::PerfectOctave => 12,
        }
    }
}

impl From<u8> for Interval {
    fn from(value: u8) -> Interval {
        match value {
            0 => Interval::PerfectUnission,
            1 => Interval::MinorSecond,
            2 => Interval::MajorSecond,
            3 => Interval::MinorThird,
            4 => Interval::MajorThird,
            5 => Interval::PerfectFourth,
            6 => Interval::DiminishedFifth,
            7 => Interval::PerfectFifth,
            8 => Interval::MinorSixth,
            9 => Interval::MajorSixth,
            10 => Interval::MinorSeventh,
            11 => Interval::MajorSeventh,
            12 => Interval::PerfectOctave,
            _ => Interval::PerfectOctave,
        }
    }
}
