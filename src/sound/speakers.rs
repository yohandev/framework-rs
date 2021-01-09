use super::iter::AnySampleIterator;

/// output endpoint for audio
pub struct Speakers
{
    /// track currently playing
    cur: Track,
}

/// sample iterator that could (or not) exist
type Track = Option<Box<dyn AnySampleIterator + Send + Sync>>;

/// errors that could occur upon speakers creation
pub enum SpeakersErr
{
    /// indicates that `cpal::Host::default_output_device`
    /// returned `None`
    NoOutputDevice,
    /// indicates that `cpal::DeviceTrait::default_output_config`
    /// returned an error
    DefaultStreamConfigError(cpal::DefaultStreamConfigError),
}

impl Speakers
{
    /// connects a new `Speakers` instance to the device
    /// endpoint, and starts an output stream
    ///
    /// tldr; play some sound!
    fn new() -> Result<Self, SpeakersErr>
    {
        use cpal::traits::*;

        let host = cpal::default_host();
        let devc = host
            .default_output_device()
            .ok_or(SpeakersErr::NoOutputDevice)?;
        let conf = devc
            .default_output_config()
            .map_err(|e| SpeakersErr::DefaultStreamConfigError(e))?;
        
        Ok(Self { cur: None })
    }
}

/// tests that Audio is still Send + Sync(remove this once impl is done)
fn _test_sync()
{
    fn is_send<T: Send>() { }
    fn is_sync<T: Sync>() { }

    is_sync::<Speakers>(); // compiles only if true
    is_send::<Speakers>();
}