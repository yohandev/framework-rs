/// manages audio and connects to output device(speaker)
pub struct Audio
{
    // ...
}

/// represents a buffer/iterator of samples over 1-n channels
pub trait AudioClip<S>: Iterator<Item = S> where S: cpal::Sample
{
    // ...
}

impl Audio
{
    /// add an audio clip to audio output queue
    pub fn play<S: cpal::Sample>(&mut self, _clip: std::rc::Rc<impl AudioClip<S>>)
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