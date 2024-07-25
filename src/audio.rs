use rodio::{Decoder, OutputStream, Sink};
use std::io::{Read, Seek};

pub fn play_metronome<R>(audio: R)
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
