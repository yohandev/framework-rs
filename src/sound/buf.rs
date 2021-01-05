/// represents a buffer of interleaved audio samples
pub struct AudioBuf<S>
{
    /// interleaved samples [Channel1Sample1, C2S1, C1S2, C2S2, ...]
    samples: Box<[S]>,

    /// number of channels in this audio buffer
    channels: usize,
    /// sampling rate of the audio within this buffer
    sample_rate: u32,
}

impl<S> AudioBuf<S>
{
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