use std::sync::mpsc::{ channel, Sender };

use super::iter::AnySampleIterator;
use crate::sound::SampleType;

/// output endpoint for audio
pub struct Speakers
{
    /// internal cpal output stream
    _stream: cpal::Stream,

    /// send tracks to the audio thread to play
    send: Sender<Track>,
    /// config: (s)a(mp)le (t)ype
    smpt: SampleType,
}

/// sample iterator now playing
type Track = Box<dyn AnySampleIterator>;

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
        let mut track = Option::<Track>::None;
        // track next-up(send, recv)
        let (send, recv) = channel();
        
        let _stream = devc.build_output_stream_raw(&conf, smpf, move |data, _|
        {
            // check for new track
            if let Ok(new) = recv.try_recv()
            {
                track = Some(new);
            }

            // write
            if let Some(curr) = &mut track
            {
                // TODO: do something with whether track is done or not
                curr.write_samples(data.bytes_mut(), smpt);
            }
            // (temporary) silence
            else
            {
                fn silence<T: crate::sound::Sample>(data: &mut cpal::Data)
                {
                    data
                        .as_slice_mut::<T>()
                        .unwrap()
                        .iter_mut()
                        .for_each(|i| *i = T::SILENCE);
                }

                match smpf
                {
                    cpal::SampleFormat::I16 => silence::<i16>(data),
                    cpal::SampleFormat::U16 => silence::<u16>(data),
                    cpal::SampleFormat::F32 => silence::<f32>(data),
                }
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
    pub fn play(&self, track: impl AnySampleIterator)
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