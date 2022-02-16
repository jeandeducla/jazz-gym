mod notes;
mod player;
mod polysines;
mod sinewave;
mod source;

use notes::Notes;
use player::Player;
use polysines::PolySines;
use sinewave::SineWave;

fn main() {
    let sample_rate = 44100;
    let player = Player::new(sample_rate);

    let num_points = 44100 * 2;
    let c4 = SineWave::new(Notes::Eb4.freqency(), sample_rate as f32, num_points);
    let e4 = SineWave::new(Notes::C4.freqency(), sample_rate as f32, num_points);
    let poly = PolySines::new(c4, e4);

    player.play(poly.collect::<Vec<i16>>().as_ref());
}
