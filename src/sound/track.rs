use std::sync::Arc;

use rodio::{ Sink, Sample };

/// represents an audio track, with playback, volume,
/// and other audio controls. 
pub struct Track<S: Sample>
{
    sink: Sink,

    /// this audio file's interleaved samples
    samples: Arc<[S]>,
    /// number of channels in this file
    channels: usize,
    /// sample rate of this file
    sample_rate: usize,
}

impl<S: Sample> Track<S>
{
    pub fn open()
    {

    }
}