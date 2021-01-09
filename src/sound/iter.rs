use crate::sound::{ Sample, SampleType };

/// an iterator that specifically yields samples to be
/// written to an audio output stream
pub trait SampleIterator
{
    /// format of the sample returned by this iterator
    type Format: Sample;

    /// get the next sample (and?) increment self for
    /// the next iteration
    fn next_sample(&mut self) -> Option<Self::Format>;
}

/// abstraction over `SampleIterator` to permit dynamic
/// types
pub(crate) trait AnySampleIterator
{
    /// write `self`'s next sample directly to the stream.
    /// assume `stream` starts at the correct cursor(index 0)
    /// and has enough room for this sample(length >= sizeof(format))
    ///
    /// returns `true` if `self as SampleIterator` is done
    fn write_next_sample(&mut self, stream: &mut [u8], format: SampleType) -> bool;
}

impl<T: SampleIterator<Format = f32>> AnySampleIterator for T
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
                    F32 => stream[0..4].copy_from_slice(&s.to_ne_bytes()),
                    I16 => {}
                    U16 => {}
                }
                false
            }
        }
    }
}