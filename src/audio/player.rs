use rodio::Sink;
use rodio::{OutputStream, OutputStreamHandle};

pub struct Player {
    stream: OutputStream,
    pub stream_handle: OutputStreamHandle,
}

impl Player {
    pub fn new() -> Self {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        Player {
            stream,
            stream_handle,
        }
    }

    pub fn play(self, source: &Vec<Sink>) {
        source.iter().for_each(|s| s.sleep_until_end());
    }
}
