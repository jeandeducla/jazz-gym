use alsa::pcm::{Access, Format, HwParams, State, PCM};
use alsa::{Direction, ValueOr};

mod notes;
mod polysines;
mod sinewave;
mod source;

use notes::Notes;
use polysines::PolySines;
use sinewave::SineWave;

fn main() {
    // Open default playback device
    let pcm = PCM::new("default", Direction::Playback, false).unwrap();

    // Set hardware parameters: 44100 Hz / Mono / 16 bit
    let sample_rate = 44100;
    let hwp = HwParams::any(&pcm).unwrap();
    hwp.set_channels(1).unwrap();
    hwp.set_rate(sample_rate, ValueOr::Nearest).unwrap();
    hwp.set_format(Format::s16()).unwrap();
    hwp.set_access(Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    let io = pcm.io_i16().unwrap();

    // Make sure we don't start the stream too early
    let hwp = pcm.hw_params_current().unwrap();
    let swp = pcm.sw_params_current().unwrap();
    swp.set_start_threshold(hwp.get_buffer_size().unwrap())
        .unwrap();
    pcm.sw_params(&swp).unwrap();

    // Make a sine wave
    let num_points = 44100 * 2;
    let c4 = SineWave::new(Notes::C4.freqency(), sample_rate as f32, num_points);
    let e4 = SineWave::new(Notes::E4.freqency(), sample_rate as f32, num_points);
    let third = PolySines::new(c4, e4);

    let g4 = SineWave::new(Notes::G4.freqency(), sample_rate as f32, num_points);
    let b4 = SineWave::new(Notes::B4.freqency(), sample_rate as f32, num_points);
    let top = PolySines::new(g4, b4);

    let poly = PolySines::new(third, top);
    let a4 = SineWave::new(Notes::A4.freqency(), sample_rate as f32, num_points);
    let poly = PolySines::new(poly, a4);

    // TODO: should return the current_point
    // let n = a4.num_points();
    assert_eq!(io.writei(poly.collect::<Vec<i16>>().as_ref()).unwrap(), 2);
    // assert_eq!(io.writei(&coucou).unwrap(), n);

    // In case the buffer was larger than 2 seconds, start the stream manually.
    println!("{:?}", pcm.state());
    if pcm.state() != State::Running {
        println!("coucou");
        pcm.start().unwrap()
    };
    // Wait for the stream to finish playback.

    pcm.drain().unwrap();
}
