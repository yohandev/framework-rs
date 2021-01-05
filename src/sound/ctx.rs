use crate::sound::{ Sample, StreamWriter };

/// manages audio and connects to endpoints(out speaker, in microphone)
pub struct AudioCtx
{
    // ...
}

impl AudioCtx
{
    pub fn play<S: Sample, T: StreamWriter<S>>(&mut self, _clip: &impl Into<T>)
    {
        // ...
    }
}

impl Default for AudioCtx
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

    is_sync::<AudioCtx>(); // compiles only if true
    is_send::<AudioCtx>();
}