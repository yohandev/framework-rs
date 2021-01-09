use std::sync::Arc;

use super::Sample;

/// represents a sound file that can be loaded at
/// runtime
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundFile<S: Sample>
{
    /// this audio file's interleaved samples
    samples: Arc<[S]>,

    /// number of channels in this file
    channels: usize,
    /// sample rate of this file
    sample_rate: usize,
}

/// an error while reading an audio file, either io or format
pub type SoundFileError = audrey::read::ReadError;

impl<S: Sample> SoundFile<S>
{
    /// Attempts to open an `AudioFile` at the specified `Path`.
    ///
    /// The format is determined from the path's file extension.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, SoundFileError>
    {
        // read the audio file
        let mut reader = audrey::open(path)?;

        // collet the samples
        let samples = reader
            .samples()
            .map(Result::unwrap)
            .collect::<Arc<[_]>>();
        
        // description
        let desc = reader.description();
        let channels = desc.channel_count() as usize;
        let sample_rate = desc.sample_rate() as usize;
        
        Ok(Self { samples, channels, sample_rate })
    }

    /// number of channels in this file
    pub fn channels(&self) -> usize
    {
        self.channels
    }

    /// sample rate of this file
    pub fn sample_rate(&self) -> usize
    {
        self.sample_rate
    }

    /// iterate this audio file's interleaved samples
    pub fn samples(&self) -> impl Iterator<Item = &S>
    {
        self.samples.iter()
    }

    /// iterate this audio file's frames
    pub fn frames(&self) -> impl Iterator<Item = &[S]>
    {
        self.samples
            .chunks_exact(self.channels)
    }
}