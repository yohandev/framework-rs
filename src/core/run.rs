use std::collections::HashMap;

use winit::event_loop::{ ControlFlow, EventLoop, EventLoopWindowTarget as WindowTarget };
use winit::window::WindowId;

use crate::input::{ Input, ProcessedEvent };
use crate::core::{ Sketch, Time, App };
use crate::draw::Window;

use super::app::AppRequest;

/// run the sketch, hyjacking the main thread until the
/// window is closed
pub fn run<T: Sketch>()
{
    // create event loop
    let events = EventLoop::new();

    // windows
    let mut windows = HashMap::<WindowId, Window>::new();

    // inputs
    let mut input = Input::new();
    let mut time = Time::new();
    
    // callbacks to avoid code duplication
    let app = ||
    {
        App
        {
            input: &input,
            time: &time,
            requests: Vec::with_capacity(0),
        }
    };
    let mut process_requests = |target: &WindowTarget<()>, requests: Vec<AppRequest>|
    {
        for request in requests
        {
            match request
            {
                AppRequest::CreateCanvas(title, size) =>
                {
                    let window = Window::new(target, title, size);

                    windows.insert(window.winit.id(), window);
                }
                AppRequest::CloseCanvas(id) =>
                {
                    windows.remove(&id.0);
                }
            }
        }
    };
    
    // create the `Sketch`
    let mut sketch =
    {
        // setup with an `App`
        let mut app = app();
        let sketch = T::setup(&mut app);

        process_requests(&events, app.requests);

        sketch
    };

    events.run(move |event, _, control_flow|
    {
        match input.process(event)
        {
            // notify `Pixels` of the window resize
            ProcessedEvent::WindowResized(id, (w, h)) =>
            {
                windows[&id].pixels.resize(w, h);
            }
            // close window and quit if needed
            ProcessedEvent::WindowClose(id) =>
            {
                // stop keeping track of and drop window
                windows.remove(&id);

                // quit?
                if windows.is_empty()
                {
                    return *control_flow = ControlFlow::Exit;
                }
            }
            // render to the given window
            ProcessedEvent::ShouldRender(id) =>
            {
                // get the frame
                let window = windows[&id];
                
                // update buffer
                sketch.draw(&mut window.get_frame());

                // render
                if window.pixels.render().is_err()
                {
                    return *control_flow = ControlFlow::Exit;
                }
            }
            // update the Sketch state and request new events
            ProcessedEvent::ShouldUpdate =>
            {
                // update time
                time.update();

                // update state
                sketch.update(&mut app());
                
                // request a redraw on each window
                for (_, window) in windows.iter_mut()
                {
                    window.winit.request_redraw();
                }
            }
            ProcessedEvent::None => { }
        }
    });
}