use super::*;

use rodio::{Decoder, OutputStream, Sink};
use std::io::{BufReader, Cursor, Read, Seek};

pub struct Metronome {
    pub bpm: f64,
    pub playing: bool,
}

impl Metronome {
    pub const fn new() -> Self {
        Self {
            bpm: DEFAULT_BPM,
            playing: false,
        }
    }

    pub fn start(&mut self) {
        self.playing = true;

        let audio = include_bytes!("sounds/fl-metronome-hat.wav");
        let audio = BufReader::new(Cursor::new(audio));
        Self::play_hit(audio)
    }

    fn play_hit<R>(audio: R)
    where
        R: Read + Seek + Send + Sync + 'static,
    {
        // _stream must live as long as the sink
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        let source = Decoder::new(audio).unwrap();
        sink.append(source);

        sink.sleep_until_end();
    }
}
