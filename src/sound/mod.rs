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

mod speakers;
mod silence;
mod sample;
mod track;
mod file;

pub use speakers::Speakers;
pub use sample::{ Sample, SampleType };
pub use track::Track;
pub use file::SoundFile;