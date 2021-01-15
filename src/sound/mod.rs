mod sample;
mod track;

pub use self::sample::{ Sample, SampleType };
pub use self::track::Track;

/// audio context. this is a dead simple wrapper around `rodio`'s
/// types
pub struct Audio
{
    _stream: rodio::OutputStream,
    handle: rodio::OutputStreamHandle,
}

impl Audio
{
    /// create a new audio context and connect to the endpoint,
    /// maintaining that connection until dropped
    pub(crate) fn new() -> Self
    {
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();

        Self { _stream, handle }
    }
}