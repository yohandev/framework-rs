use std::sync::Arc;

use super::{ Sample, Track };

/// represents a sound file that can be loaded at
/// runtime
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundFile<S: Sample>
{
    /// this audio file's interleaved samples
    samples: Arc<[S]>,

    /// number of channels in this file
    channels: usize,
    /// sample rate of this file
    sample_rate: usize,
}

struct SoundFileTrack<S: Sample>
{
    /// this audio file's interleaved samples
    samples: Arc<[S]>,
    /// number of channels in this file
    channels: usize,
    /// next index
    index: usize,

    /// sampler function that determines how to interpret
    /// `self.samples` based on `self.channels` and the
    /// `Speakers`' channel count
    sampler: fn(&[S], &mut usize, &mut usize) -> S,
    /// arbitrary variable used(or not) by the sampler
    sampler_cache: usize,
}

/// an error while reading an audio file, either io or format
pub type SoundFileError = audrey::read::ReadError;

impl<S: Sample> SoundFile<S>
{
    /// Attempts to open an `AudioFile` at the specified `Path`.
    ///
    /// The format is determined from the path's file extension.
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, SoundFileError>
    {
        // read the audio file
        let mut reader = audrey::open(path)?;

        // collet the samples
        let samples = reader
            .samples()
            .map(Result::unwrap)
            .collect::<Arc<[_]>>();
        
        // description
        let desc = reader.description();
        let channels = desc.channel_count() as usize;
        let sample_rate = desc.sample_rate() as usize;
        
        Ok(Self { samples, channels, sample_rate })
    }

    /// number of channels in this file
    pub fn channels(&self) -> usize
    {
        self.channels
    }

    /// sample rate of this file
    pub fn sample_rate(&self) -> usize
    {
        self.sample_rate
    }

    /// iterate this audio file's interleaved samples
    pub fn samples(&self) -> impl Iterator<Item = &S>
    {
        self.samples.iter()
    }

    /// iterate this audio file's frames
    pub fn frames(&self) -> impl Iterator<Item = &[S]>
    {
        self.samples
            .chunks_exact(self.channels)
    }
}

impl<S: Sample> Track for SoundFileTrack<S>
{
    type Format = S;

    fn next_sample(&mut self) -> Option<Self::Format>
    {
        // done
        if self.index == self.samples.len()
        {
            None
        }
        // call the sampler
        else
        {
            Some((self.sampler)(&self.samples, &mut self.index, &mut self.sampler_cache))
        }
    }

    fn tune(&mut self, channels: usize, _: usize)
    {
        // sampling function
        self.sampler = match (self.channels, channels)
        {
            // output == input
            (_, _) if self.channels == channels => samplers::identity::<S>,
            // input = _, output = 1
            (_, 1) => 
            {
                self.sampler_cache = self.channels;

                samplers::sum::<S>
            },
            // input = 1, output = _
            (1, _) =>
            {
                self.sampler_cache = channels;

                samplers::copy::<S>
            }
            // ???
            (_, _) => panic!("not implemented!")
        };
        // reset index
        self.index = 0;
    }
}

/// sampling functions that map an audio file with its
/// specifications to the `Speakers` output stream
mod samplers
{
    use crate::sound::Sample;

    /// same number of channels in file as output
    pub fn identity<S: Sample>(samples: &[S], index: &mut usize, _: &mut usize) -> S
    {
        // increment
        *index += 1;

        // next sample
        samples[*index - 1]
    }

    /// one output channel, `$arg3` audio file channels
    pub fn sum<S: Sample>(samples: &[S], index: &mut usize, channels: &mut usize) -> S
    {
        // start bound
        let start = *index;
        // end bound
        *index += *channels;

        // sum samples
        let mut sum = S::zero();
        for sample in samples[start..*index].iter()
        {
            sum += *sample;
        }
        sum
    }   
    
    /// one audio file channel, `$arg3` output channels
    pub fn copy<S: Sample>(samples: &[S], index: &mut usize, channels: &mut usize) -> S
    {
        // only increment if on next frame
        if *index % *channels == 0
        {
            // increment
            *index += 1;
        }

        // next sample
        samples[*index - 1]
    }
}