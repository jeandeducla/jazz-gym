use std::fmt::{self, Display};
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum Interval {
    PerfectUnison,
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
            Interval::PerfectUnison => 0,
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
            "P1" => Ok(Interval::PerfectUnison),
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
            Interval::PerfectUnison => 0,
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
            0 => Interval::PerfectUnison,
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

impl Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Interval::PerfectUnison => "Perfect Unission (P1)",
            Interval::MinorSecond => "Minor Second (m2)",
            Interval::MajorSecond => "Major Second (M2)",
            Interval::MinorThird => "Minor Third (m3)",
            Interval::MajorThird => "Major Third (M3)",
            Interval::PerfectFourth => "Perfect Fourth (P4)",
            Interval::DiminishedFifth => "Diminished Fifth (d5)",
            Interval::PerfectFifth => "Perfect Fifth (P5)",
            Interval::MinorSixth => "Minor Sixth (m6)",
            Interval::MajorSixth => "Major Sixth (M6)",
            Interval::MinorSeventh => "Minor Seventh (m7)",
            Interval::MajorSeventh => "Major Seventh (M7)",
            Interval::PerfectOctave => "Perfect Octave (P8)",
        };
        write!(f, "{}", s)
    }
}
