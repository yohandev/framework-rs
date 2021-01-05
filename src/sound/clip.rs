use crate::sound::{ Sample, AudioBuf };

/// represents a wav, ogg, caf, or flac file
pub type Clip<S> = AudioBuf<S, Vec<S>>;

/// an error while reading an audio file, either io or format
pub type AudioFileError = audrey::read::ReadError;

impl<S: Sample + audrey::read::Sample> Clip<S>
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
        let channel_count = desc.channel_count() as usize;
        let sample_rate = desc.sample_rate();
        
        Ok(Self::new(samples, channel_count, sample_rate))
    }
}