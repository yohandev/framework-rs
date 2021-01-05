use crate::sound::AudioBuf;

/// represents an iterator of samples over 1-n channels
pub trait AudioClip<S: cpal::Sample>
{
    /// write this clip's samples to the audio buffer
    fn write(&mut self, buf: &mut AudioBuf<S>);
}