use rustyline::Editor;
use rustyline::Result;

mod audio;
mod gym;
mod music;

use crate::gym::{welcome, Session};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new();
    welcome();
    let mut session = Session::new();
    let _ = session.start(&mut rl);
    Ok(())
}
