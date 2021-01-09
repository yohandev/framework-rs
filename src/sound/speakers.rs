use std::sync::mpsc::{ channel, Sender };

use crate::sound::{ SampleType, silence, track };

/// output endpoint for audio
pub struct Speakers
{
    /// internal cpal output stream
    _stream: cpal::Stream,

    /// send tracks to the audio thread to play
    send: Sender<DynTrack>,
    /// config: (s)a(mp)le (t)ype
    smpt: SampleType,
}

/// sample iterator now playing
type DynTrack = Box<dyn track::RawTrack>;

/// errors that could occur upon speakers creation
#[derive(Debug)]
pub enum SpeakersErr
{
    /// indicates that `cpal::Host::default_output_device`
    /// returned `None`
    NoOutputDevice,
    /// indicates that `cpal::DeviceTrait::default_output_config`
    /// returned an error
    DefaultStreamConfigError(cpal::DefaultStreamConfigError),
    /// indicates that `cpal::DeviceTrait::build_output_stream_raw`
    /// returned an error
    BuildStreamError(cpal::BuildStreamError),
}

use cpal::traits::*;

impl Speakers
{
    /// connects a new `Speakers` instance to the device
    /// endpoint, and starts an output stream
    ///
    /// tldr; play some sound!
    pub fn new() -> Result<Self, SpeakersErr>
    {
        // cpal init
        let host = cpal::default_host();
        let devc = host
            .default_output_device()
            .ok_or(SpeakersErr::NoOutputDevice)?;
        let conf = devc
            .default_output_config()
            .map_err(|e| SpeakersErr::DefaultStreamConfigError(e))?;
        let smpf = conf.sample_format();
        let smpt = smpf.into();
        let conf = conf.into();
        let e_fn = |e| eprintln!("an error occured on stream: {}", e);
        
        // track now playing
        let mut track = Option::<DynTrack>::None;
        // track next-up(send, recv)
        let (send, recv) = channel();
        
        let _stream = devc.build_output_stream_raw(&conf, smpf, move |stream, _|
        {
            // check for new track
            if let Ok(new) = recv.try_recv()
            {
                track = Some(new);
            }

            // write
            if let Some(curr) = &mut track
            {
                // check if done
                if curr.write_samples(stream.bytes_mut(), smpt)
                {
                    track = None;
                }
            }
            // silence
            else
            {
                silence::write_silence(stream, smpt);
            }

        }, e_fn).map_err(|e| SpeakersErr::BuildStreamError(e))?;

        Ok(Self { _stream, send, smpt })
    }

    /// the type of samples used by these speakers
    pub fn sample_type(&self) -> SampleType
    {
        self.smpt
    }

    /// change the track currently playing
    pub fn play(&self, track: impl track::RawTrack)
    {
        self.send
            .send(Box::new(track))
            .unwrap();
    }
}

// /// tests that Audio is still Send + Sync(remove this once impl is done)
// fn _test_sync()
// {
//     fn is_send<T: Send>() { }
//     fn is_sync<T: Sync>() { }

//     is_sync::<Speakers>(); // compiles only if true
//     is_send::<Speakers>();
// }