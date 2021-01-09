use super::iter::AnySampleIterator;

/// output endpoint for audio
pub struct Speakers
{
    /// track currently playing
    _cur: Track,
}

/// sample iterator that could (or not) exist
type Track = Option<Box<dyn AnySampleIterator + Send + Sync>>;

impl Speakers
{
    // ...
}

/// tests that Audio is still Send + Sync(remove this once impl is done)
fn _test_sync()
{
    fn is_send<T: Send>() { }
    fn is_sync<T: Sync>() { }

    is_sync::<Speakers>(); // compiles only if true
    is_send::<Speakers>();
}