pub mod input;
pub mod sound;
pub mod draw;
pub mod util;
pub mod math;

mod sketch;
mod app;

pub use crate::sketch::Sketch;
pub use crate::app::App;

pub mod prelude
{
    pub use crate::math::vec::
    {
        Vec2,
        Vec3,
        Vec4,
        Rgba,
        Extent2,
    };

    pub use crate::sound::
    {
        Track,
    };
    pub use crate::draw::
    {
        ParallelIterator,
        Image,
        Canvas,
    };
    pub use crate::
    {
        run,
        Sketch,
        App
    };
}

// run the sketch, hyjacking the main thread until all the
/// windows are closed
pub fn run<T: sketch::Sketch>()
{
    // create event loop
    let events = winit::event_loop::EventLoop::new();

    // app & sketch
    let mut app = app::App::new();
    let mut sketch = T::setup(&mut app);
    
    // create initial windows
    app.process_requests(&events);

    events.run(move |event, window_target, control_flow|
    {
        app.process_event(&mut sketch, event, window_target, control_flow);
    });
}