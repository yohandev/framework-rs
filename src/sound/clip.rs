use crate::sound::{ AudioBuf, Sample };

/// an `AudioBuf`
pub type AudioStream<'a, S> = AudioBuf<S, &'a mut [S]>;

/// represents an iterator of samples over 1-n channels
pub trait AudioClip<S: Sample>
{
    /// type being used for each sample
    type Sample: Sample;

    /// write this clip's samples to the audio buffer
    fn write(&mut self, stream: &mut AudioStream<Self::Sample>);
}