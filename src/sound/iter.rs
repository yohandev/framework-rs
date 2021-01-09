use crate::sound::{ Sample, SampleType };

/// an iterator that specifically yields samples to be
/// written to an audio output stream
pub trait SampleIterator: Send + Sync + 'static
{
    /// format of the sample returned by this iterator
    type Format: Sample;

    /// get the next sample (and?) increment self for
    /// the next iteration
    fn next_sample(&mut self) -> Option<Self::Format>;
}

/// abstraction over `SampleIterator` to permit dynamic
/// types
pub trait AnySampleIterator: Send + Sync + 'static
{
    /// write `self`'s next sample directly to the stream.
    /// assume `stream` starts at the correct cursor(index 0)
    /// and has enough room for this sample(length >= sizeof(format))
    ///
    /// returns `true` if `self as SampleIterator` is done
    fn write_next_sample(&mut self, stream: &mut [u8], format: SampleType) -> bool;
}

impl<S: Sample, T: SampleIterator<Format = S>> AnySampleIterator for T
{
    fn write_next_sample(&mut self, stream: &mut [u8], format: SampleType) -> bool
    {
        use SampleType::*;

        match self.next_sample()
        {
            // exhausted
            None => true,
            // ... or write
            Some(s) =>
            {
                match format
                {
                    F32 => stream[0..4].copy_from_slice(&s.to_f32().to_ne_bytes()),
                    I16 => stream[0..2].copy_from_slice(&s.to_i16().to_ne_bytes()),
                    U16 => stream[0..2].copy_from_slice(&s.to_u16().to_ne_bytes()),
                }
                false
            }
        }
    }
}