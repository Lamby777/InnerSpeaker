use super::*;

use rodio::{Decoder, OutputStream, Sink};
use std::io::{BufReader, Cursor, Read, Seek};
use std::time::Duration;

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

    pub fn start(&mut self, on_off: Receiver<bool>) {
        self.playing = true;

        let audio = include_bytes!("sounds/fl-metronome-hat.wav");
        let audio = BufReader::new(Cursor::new(audio));
        thread::spawn(|| Self::play_hit(audio));

        thread::sleep(Duration::from_millis(60000 / (self.bpm as u64)));

        if let Ok(new_state) = on_off.try_recv() {
            self.playing = new_state;
        }

        if self.playing {
            self.start(on_off);
        }
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
