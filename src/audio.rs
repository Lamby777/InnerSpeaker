/*
* Definitions that prob don't belong in main.
*/

use rodio::{Decoder, OutputStream, Sink};
use std::io::{BufReader, Cursor};

pub fn play_metronome() {
    let audio = include_bytes!("sounds/fl-metronome-hat.wav");
    let audio = BufReader::new(Cursor::new(audio));

    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = Decoder::new(audio).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}
