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

    pub fn from_config(config: &Config) -> Self {
        Self {
            bpm: config.bpm,
            measure_len: config.measure_len,
            nth_beat: 0,
        }
    }

    pub fn start(metronome: &RwLock<Metronome>, rx: Receiver<bool>) {
        loop {
            // if the channel receives a `false`, block until it receives a `true`
            if let Ok(false) = rx.try_recv() {
                loop {
                    if let Ok(true) = rx.recv() {
                        metronome.write().unwrap().nth_beat = 0;
                        break;
                    }
                }
            }

            let bpm = {
                let mut metronome = metronome.write().unwrap();
                metronome.hit();

                metronome.nth_beat += 1;
                if metronome.nth_beat >= metronome.measure_len {
                    metronome.nth_beat = 0;
                }

                metronome.bpm
            };

            thread::sleep(bpm_to_duration(bpm));
        }
    }

    pub fn hit(&self) {
        // don't play a high pitched sound if it's every single beat
        let first = self.nth_beat == 0 && self.measure_len > 1;

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

        if first {
            sink.set_speed(FIRST_BEAT_SPEED);
        }

        let source = Decoder::new(audio).unwrap();
        sink.append(source);

        sink.sleep_until_end();
    }
}

fn bpm_to_duration(bpm: f64) -> Duration {
    Duration::from_millis(60000 / (bpm as u64))
}
