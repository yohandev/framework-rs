use winit::event_loop::EventLoop;

use crate::core::{ Sketch, App };


/// run the sketch, hyjacking the main thread until the
/// window is closed
pub fn run<T: Sketch>()
{
    // create event loop
    let events = EventLoop::new();

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