use alsa::pcm::{Access, Format, HwParams, PCM};
use alsa::{Direction, ValueOr};

pub struct Player {
    sample_rate: u32,
}

impl Player {
    pub fn new(sample_rate: u32) -> Self {
        Player { sample_rate }
    }

    pub fn play(self, source: &[i16]) {
        // Open default playback device
        let pcm = PCM::new("default", Direction::Playback, false).unwrap();

        // Set hardware parameters: Mono / 16 bit
        let hwp = HwParams::any(&pcm).unwrap();
        hwp.set_channels(1).unwrap();
        hwp.set_rate(self.sample_rate, ValueOr::Nearest).unwrap();
        hwp.set_format(Format::s16()).unwrap();
        hwp.set_access(Access::RWInterleaved).unwrap();
        pcm.hw_params(&hwp).unwrap();

        // Make sure we don't start the stream too early
        let hwp = pcm.hw_params_current().unwrap();
        let swp = pcm.sw_params_current().unwrap();
        swp.set_start_threshold(hwp.get_buffer_size().unwrap())
            .unwrap();
        pcm.sw_params(&swp).unwrap();

        let io = pcm.io_i16().unwrap();
        io.writei(&source).unwrap();
        pcm.drain().unwrap();
    }
}
