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

mod write;
mod clip;
mod buf;
mod ctx;

pub use write::StreamWriter;
pub use clip::Clip;
pub use buf::AudioBuf;
pub use ctx::AudioCtx;

pub use cpal::Sample;