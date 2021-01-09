/// represents a buffer of interleaved audio samples
pub struct AudioBuf<S, T: Buf<S>>
{
    /// interleaved samples [Channel1Sample1, C2S1, C1S2, C2S2, ...]
    samples: T,

    /// number of channels in this audio buffer
    channel_count: usize,
    /// sampling rate of the audio within this buffer
    sample_rate: u32,

    phantom: std::marker::PhantomData<S>
}

/// restrictions for a type that can be used as an audio
/// buffer
pub trait Buf<S>: AsRef<[S]> + AsMut<[S]> { }

impl<S, T: Buf<S>> AudioBuf<S, T>
{
    /// create a new audio buffer from its raw parts
    pub fn new(samples: T, channel_count: usize, sample_rate: u32) -> Self
    {
        debug_assert_eq!(samples.as_ref().len() % channel_count, 0);

        let phantom = Default::default();

        Self { samples, channel_count, sample_rate, phantom }
    }

    /// number of channels in this audio buffer
    pub fn channel_count(&self) -> usize
    {
        self.channel_count
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
        self.samples.as_ref().chunks_exact(self.channel_count)
    }

    /// returns an iterator over this audio buffer's frames,
    /// where a frame is an array of samples for each channel
    pub fn frames_mut(&mut self) -> impl Iterator<Item = &mut [S]>
    {
        self.samples.as_mut().chunks_exact_mut(self.channel_count)
    }
}

/// blanket implementation
impl<S, C: AsRef<[S]> + AsMut<[S]>> Buf<S> for C { }