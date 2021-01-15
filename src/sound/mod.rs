mod sample;
mod track;

pub use self::sample::{ Sample, SampleType };
pub use self::track::Track;

pub struct Audio
{
    _stream: rodio::OutputStream,
    handle: rodio::OutputStreamHandle,
}

impl Audio
{
    pub(crate) fn new() -> Self
    {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

        Self { _stream, handle }
    }
}