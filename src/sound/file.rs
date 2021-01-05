use crate::sound::Sample;

/// represents a wav, ogg, caf, or flac file
pub struct AudioFile<S>
{
    samples: Vec<S>,

    /// number of channels in this audio buffer
    channels: usize,
    /// sampling rate of the audio within this buffer
    sample_rate: u32,
}

/// an error while reading an audio file, either io or format
pub type AudioFileError = audrey::read::ReadError;

impl<S: Sample + audrey::read::Sample> AudioFile<S>
{ 
    /// Attempts to open an `AudioFile` at the specified `Path`.
    ///
    /// The format is determined from the path's file extension.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, AudioFileError>
    {
        // read the audio file
        let mut reader = audrey::open(path)?;

        // collet the samples
        let samples = reader
            .samples()
            .map(Result::unwrap)
            .collect();
        
        // description
        let desc = reader.description();
        let channels = desc.channel_count() as usize;
        let sample_rate = desc.sample_rate();
        
        Ok(Self { samples, channels, sample_rate })
    }

    /// number of channels in this audio buffer
    pub fn channels(&self) -> usize
    {
        self.channels
    }

    /// sampling rate of the audio within this buffer
    pub fn sample_rate(&self) -> u32
    {
        self.sample_rate
    }

    /// returns an iterator over this audio buffer's frames,
    /// where a frame is an array of samples for each channel
    pub fn frames(&self) -> impl Iterator<Item = &[S]>
    {
        self.samples.chunks_exact(self.channels)
    }

    /// returns an iterator over this audio buffer's frames,
    /// where a frame is an array of samples for each channel
    pub fn frames_mut(&mut self) -> impl Iterator<Item = &mut [S]>
    {
        self.samples.chunks_exact_mut(self.channels)
    }
}