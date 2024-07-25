use rodio::{Decoder, OutputStream, Sink};
use std::io::{BufReader, Cursor, Read, Seek};

/// Will continue playing until `PLAYING` is set to false from another thread.
pub fn start_metronome() {
    let audio = include_bytes!("sounds/fl-metronome-hat.wav");
    let audio = BufReader::new(Cursor::new(audio));
    play_hit(audio)
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
