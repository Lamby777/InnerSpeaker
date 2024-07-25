use super::*;

use rodio::{Decoder, OutputStream, Sink};
use std::io::{BufReader, Cursor, Read, Seek};
use std::time::Duration;

pub struct Metronome {
    pub bpm: f64,
    pub measure_len: u8,
    pub nth_beat: u8,
}

impl Metronome {
    pub const fn new() -> Self {
        Self {
            bpm: DEFAULT_BPM,
            measure_len: DEFAULT_MEASURE_LEN,
            nth_beat: 0,
        }
    }

    pub fn start(metronome: &RwLock<Metronome>, rx: Receiver<bool>) {
        loop {
            // if the channel receives a `false`, block until it receives a `true`
            if let Ok(false) = rx.try_recv() {
                loop {
                    if let Ok(true) = rx.recv() {
                        break;
                    }
                }
            }

            let metronome = metronome.read().unwrap();
            metronome.hit(false);

            let bpm = metronome.bpm;
            drop(metronome);

            thread::sleep(Duration::from_millis(60000 / (bpm as u64)));
        }
    }

    pub fn hit(&self, first: bool) {
        let audio = include_bytes!("sounds/fl-metronome-hat.wav");
        let audio = BufReader::new(Cursor::new(audio));
        thread::spawn(move || Self::play_sound(audio, first));
    }

    fn play_sound<R>(audio: R, first: bool)
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
