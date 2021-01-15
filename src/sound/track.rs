use std::time::Duration;
use std::path::Path;
use std::sync::Arc;

use crate::sound::Sample;

use super::Audio;

/// represents an audio track, with playback, volume,
/// and other audio controls. supported formats include:
/// - flac(".flac")
/// - ogg vorbis(".ogg", ".oga")
/// - wav(".wav", ".wave")
/// - alac(".caf")
pub struct Track<S: Sample>
{
    sink: rodio::Sink,

    /// this audio file's interleaved samples
    samples: Arc<[S]>,
    /// number of channels in this track
    channel_count: usize,
    /// sample rate of this track
    sample_rate: usize,
}

/// internal track that's actually passed to the rodio
/// Sink and serves samples
struct TrackSource<S: Sample>
{
    /// keeps track of what index it's at
    /// also the number of samples read
    ind: usize,

    /// this audio file's interleaved samples
    samples: Arc<[S]>,
    /// number of channels in this track
    channel_count: u16,
    /// sample rate of this track
    sample_rate: u32,   
}

/// an error while reading an audio file, either io or format
pub type TrackError = audrey::read::ReadError;

impl<S: Sample> Track<S>
{
    /// Attempts to open a `Track` at the specified `Path`.
    ///
    /// The format is determined from the path's file extension.
    pub fn open(path: impl AsRef<Path>, audio: &Audio) -> Result<Self, TrackError>
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
        let sample_rate = desc.sample_rate() as usize;
        let channel_count = desc.channel_count() as usize;

        // sink
        let sink = rodio::Sink::try_new(&audio.handle).unwrap();
        sink.append(TrackSource
        {
            ind: 0,
            samples: Arc::clone(&samples),
            channel_count: channel_count as u16,
            sample_rate: sample_rate as u32,
        });
        // pause by default
        sink.pause();

        Ok(Self { sink, samples, channel_count, sample_rate })
    }

    /// start playing or resume playback of this track
    ///
    /// no effect if not paused
    pub fn play(&self)
    {
        self.sink.play();
    }

    /// stop playing playback of this track until `play`
    /// is called
    ///
    /// no effect if not playing
    pub fn pause(&self)
    {
        self.sink.pause();
    }

    /// is this `Track` currently paused?
    pub fn is_paused(&self) -> bool
    {
        self.sink.is_paused()
    }

    /// has this `Track` finished playing?
    pub fn done(&self) -> bool
    {
        self.sink.empty()
    }
}

impl<S: Sample> Iterator for TrackSource<S>
{
    type Item = S;

    fn next(&mut self) -> Option<S>
    {
        match self.samples.get(self.ind)
        {
            Some(&s) =>
            {
                self.ind += 1;
                Some(s)
            },
            None => None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>)
    {
        let len = self.samples.len() - self.ind;
        
        (len, Some(len))
    }
}

impl<S: Sample> ExactSizeIterator for TrackSource<S> { }

impl<S: Sample> rodio::Source for TrackSource<S>
{
    fn current_frame_len(&self) -> Option<usize>
    {
        None
    }

    fn channels(&self) -> u16
    {
        self.channel_count
    }

    fn sample_rate(&self) -> u32
    {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration>
    {
        let ms = self.len() as u64 * 1000 / (self.channel_count as u64 * self.sample_rate as u64);
        Some(Duration::from_millis(ms))
    }
}