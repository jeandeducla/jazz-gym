use rodio::PlayError;
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

    pub fn sink(&self) -> Result<Sink, PlayError> {
        Sink::try_new(&self.stream_handle)
    }

    pub fn play(&self, source: &Vec<Sink>) {
        source.iter().for_each(|s| {
            println!("pd");
            s.sleep_until_end();
        });
    }
}
