use crate::sound::{ Sample, SampleType };

/// an iterator that specifically yields samples to be
/// written to an audio output stream
pub trait Track: Send + Sync + 'static
{
    /// format of the sample returned by this iterator
    type Format: Sample;

    /// get the next sample (and?) increment self for
    /// the next iteration
    fn next_sample(&mut self) -> Option<Self::Format>;

    /// called once whenever this track is passed to a
    /// `Speakers` instance
    fn tune(&mut self, channels: usize, sample_rate: usize);
}

/// abstraction over `Track` to permit dynamic
/// types. it works directly with bytes and
/// has access to the entire stream segment, rather
/// than working like an iterator
pub trait RawTrack: Send + Sync + 'static
{
    /// write `self`'s next sample directly to the stream.
    /// keep on writing until either `self as Track`
    /// is exhausted or the end of `stream` is reached.
    ///
    /// assumes `stream % sizeof(format) == 0`
    ///
    /// returns `true` if `self as Track` is done
    fn write_samples(&mut self, stream: &mut [u8], format: SampleType) -> bool;

    /// called once whenever this track is passed to a
    /// `Speakers` instance
    fn tune(&mut self, channels: usize, sample_rate: usize);
}

impl<S: Sample, T: Track<Format = S>> RawTrack for T
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

    fn tune(&mut self, channels: usize, sample_rate: usize)
    {
        Track::tune(self, channels, sample_rate);
    }
}