use crate::sound::{ AudioBuf, Sample };

/// represents an iterator of samples over 1-n channels
pub trait AudioClip<S: Sample>
{
    /// write this clip's samples to the audio buffer
    fn write(&mut self, buf: &mut AudioBuf<S>);
}