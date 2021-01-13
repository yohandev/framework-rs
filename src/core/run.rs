use std::collections::HashMap;

use winit::event_loop::{ ControlFlow, EventLoop };
use winit::window::WindowId;
use winit::event::Event;

use crate::input::{ Input, ProcessedEvent };
use crate::draw::{ Canvas, Window };
use crate::core::{ Sketch, Time };

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
    
    // create state
    let mut state = T::setup(app);

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
                state.draw(&mut window.get_frame());

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
                state.update(/* &mut App */);
                
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