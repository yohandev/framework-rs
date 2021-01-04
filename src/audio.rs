//! # framework-rs::sound
//! ```
//! use framework::sound::*;
//!
//! let out = Audio::default();
//! let wav = AudioFile::open("duck_song.wav");
//!
//! out.play(&wav); // takes a reference to an AudioFile
//!                 // internally, audiofile has an Rc/Arc
//!                 // to the samples, and each .play creates
//!                 // a wrapper iterator around those samples
//! for sample in wav.samples()
//! {
//!     // do whatever. this is legal even though the sound is
//!     // playing
//! }
//!
//! let _ = wav.samples_mut();  // <- no such method. this is a
//!                             // design decision to enable looping
//!                             // and playback simultaneously
//! ```

use crate::util::RefCounted;

/// manages audio and connects to output device(speaker)
pub struct Audio
{
    //clips: Vec<AudioClip>
}

/// represents a buffer/iterator of samples over 1-n channels
pub trait AudioClip<S>: Iterator<Item = S> where S: cpal::Sample
{
    /// the number of channels(1 = mono, 2 = stereo, etc.) in this clip
    fn channel_count() -> usize;

    /// 
    fn sampling_rate() -> usize;

    /// write this clip's samples to the audio buffer
    fn write(&mut self, buf: AudioBuf<S>);
}

pub struct AudioBuf<S>
{
    /// interleaved samples [Channel1Sample1, C2S1, C1S2, C2S2, ...]
    samples: Box<[S]>,

    /// number of channels in this audio buffer
    channels: usize,
    /// sampling rate
    sample_rate: u32,
}

impl Audio
{
    /// add an audio clip to audio output queue
    pub fn play<S: cpal::Sample, C: AudioClip<S>>(&mut self, _clip: impl RefCounted<C>)
    {
        todo!()
    }
}

impl Default for Audio
{
    fn default() -> Self
    {
        todo!()
    }
}

impl<S> AudioBuf<S>
{
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

fn _test_sync()
{
    fn is_sync<T: Sync>() { }

    is_sync::<Audio>(); // compiles only if true
}