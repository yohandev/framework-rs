use crate::sound::{ AudioBuf, Sample };

/// an `AudioBuf` wrapped around the system audio stream
pub type Stream<'a, S> = AudioBuf<S, &'a mut [S]>;

/// trait for types that write to the output stream
pub trait StreamWriter
{
    /// type being used for each sample
    type Sample: Sample;

    /// write this clip's samples to the audio buffer
    fn write(&mut self, stream: &mut Stream<Self::Sample>);
}