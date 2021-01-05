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

mod clip;
mod file;
mod buf;

pub use clip::AudioClip;
pub use file::AudioFile;
pub use buf::AudioBuf;

pub use cpal::Sample;

/// manages audio and connects to endpoints(out speaker, in microphone)
pub struct Audio
{
    // ...
}

impl Audio
{
    pub fn play<S: Sample, C: AudioClip<S>>(&mut self, _clip: &impl Into<C>)
    {

    }
}

impl Default for Audio
{
    fn default() -> Self
    {
        todo!()
    }
}

/// tests that Audio is still Send + Sync(remove this once impl is done)
fn _test_sync()
{
    fn is_sync<T: Sync>() { }
    fn is_send<T: Send>() { }

    is_sync::<Audio>(); // compiles only if true
    is_send::<Audio>();
}