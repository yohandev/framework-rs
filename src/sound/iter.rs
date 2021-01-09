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
    /// keep on writing until either `self as SampleIterator`
    /// is exhausted or the end of `stream` is reached.
    ///
    /// assumes `stream % sizeof(format) == 0`
    ///
    /// returns `true` if `self as SampleIterator` is done
    fn write_samples(&mut self, stream: &mut [u8], format: SampleType) -> bool;
}

impl<S: Sample, T: SampleIterator<Format = S>> AnySampleIterator for T
{
    fn write_samples(&mut self, mut stream: &mut [u8], format: SampleType) -> bool
    {
        macro_rules! write_samples
        {
            ($ty:ident, $siz:literal) =>
            {
                loop
                {
                    // done with stream segment but not track
                    if stream.is_empty() { break false; }

                    // not done with track
                    if let Some(s) = self.next_sample()
                    {
                        // write next sample
                        stream[0..$siz].copy_from_slice(&s.$ty().to_ne_bytes());

                        // next iteration
                        stream = &mut stream[$siz..];
                    }
                    // done with track
                    else
                    {
                        break true;
                    }
                }
            }
        }

        use SampleType::*;

        // actually write
        match format
        {
            F32 => write_samples!(to_f32, 4),
            I16 => write_samples!(to_i16, 2),
            U16 => write_samples!(to_u16, 2),
        }
    }
}