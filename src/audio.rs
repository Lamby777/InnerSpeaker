use super::*;

use rodio::{Decoder, OutputStream, Sink};
use std::io::{BufReader, Cursor, Read, Seek};
use std::time::Duration;

pub struct Metronome {
    pub bpm: f64,
}

impl Metronome {
    pub const fn new() -> Self {
        Self { bpm: DEFAULT_BPM }
    }

    pub fn start(metronome: &'static RwLock<Metronome>, rx: Receiver<bool>) {
        loop {
            let metronome = metronome.read().unwrap();
            metronome.hit();

            let bpm = metronome.bpm;
            drop(metronome);

            thread::sleep(Duration::from_millis(60000 / (bpm as u64)));

            // if the channel receives a `false`, stop the metronome
            if let Ok(false) = rx.try_recv() {
                return;
            }
        }
    }

    pub fn hit(&self) {
        let audio = include_bytes!("sounds/fl-metronome-hat.wav");
        let audio = BufReader::new(Cursor::new(audio));
        thread::spawn(|| Self::play_sound(audio));
    }

    fn play_sound<R>(audio: R)
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
