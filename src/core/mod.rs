mod sketch;
mod time;
mod app;

pub use self::sketch::Sketch;
pub use self::time::Time;
pub use self::app::App;

/// run the sketch, hyjacking the main thread until all the
/// windows are closed
pub fn run<T: Sketch>()
{
    // create event loop
    let events = winit::event_loop::EventLoop::new();

    // app & sketch
    let mut app = App::new();
    let mut sketch = T::setup(&mut app);
    
    // create initial windows
    app.process_requests(&events);

    events.run(move |event, window_target, control_flow|
    {
        app.process_event(&mut sketch, event, window_target, control_flow);
    });
}