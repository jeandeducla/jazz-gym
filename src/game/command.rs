use std::str::FromStr;

pub enum Command {
    Start,
    Quit,
    Replay,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "start" => Ok(Command::Start),
            "quit" => Ok(Command::Quit),
            "replay" => Ok(Command::Replay),
            _ => Err(()),
        }
    }
}

pub fn print_rules() {
    println!("  OooOoo                         .oOOOo.                 ");
    println!("      O                         .O     o                 ");
    println!("      o                         o                        ");
    println!("      O                         O                        ");
    println!("      o  .oOoO' ooOO ooOO       O   .oOOo O   o `oOOoOO. ");
    println!("      O  O   o    o    o        o.      O o   O  O  o  o ");
    println!("O     o  o   O   O    O          O.    oO O   o  o  O  O ");
    println!("`OooOO'  `OoO'o OooO OooO         `OooO'  `OoOO  O  o  o ");
    println!("                                              o          ");
    println!("                                           OoO'          ");
    println!("");
    println!("");
    println!("Jazz Gym is a terminal game to train your ear to recognize");
    println!("two-note intervals.");
    println!("You will hear a two-note interval and your goal is to ");
    println!("recognize it by typing one of the following:");
    println!(" - 'P1': for a Perfect Unission");
    println!(" - 'm2': for a Minor Second");
    println!(" - 'M2': for a Major Second");
    println!(" - 'm3': for a Minor Third");
    println!(" - 'M3': for a Major Third");
    println!(" - 'P4': for a Perfect Fourth");
    println!(" - 'P5': for a Perfect Fifth");
    println!(" - 'd5': for a Diminished Fifth");
    println!(" - 'm6': for a Minor Sixth ");
    println!(" - 'M6': for a Major Sixth");
    println!(" - 'm7': for a Minor Seventh");
    println!(" - 'M7': for a Major Seventh");
    println!(" - 'P8': for a Perfect Octave");
    println!("Do you want to start a new game? (type: 'start' to start");
    println!("and 'quit' to quit the game)");
}
