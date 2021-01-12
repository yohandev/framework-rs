use std::collections::HashMap;

use pixels::{ Pixels, SurfaceTexture };

use winit_input_helper::WinitInputHelper;

use winit::event_loop::{ ControlFlow, EventLoop };
use winit::window::WindowId;
use winit::event::Event;

use crate::core::{ Sketch, Time, window::Window };
use crate::draw::Canvas;

/// run the sketch, hyjacking the main thread until the
/// window is closed
pub fn run<T: Sketch>()
{
    // create event loop
    let events = EventLoop::new();

    // windows
    let mut windows = HashMap::<WindowId, Window>::new();

    // inputs
    let mut input = WinitInputHelper::new();
    let mut time = Time::new();
    
    // create state
    let mut state = T::setup(app);

    events.run(move |evt, _, control_flow|
    {
        // draw the current frame
        if let Event::RedrawRequested(id) = evt
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

        // handle input events
        if input.update(&evt)
        {
            // close events
            if input.quit()
            {
                return *control_flow = ControlFlow::Exit;
            }

            // resize the window
            if let Some(size) = input.window_resized()
            {
                pixels.resize(size.width, size.height);
            }

            // update state
            state.update(time.update());
            
            // request a redraw
            window.request_redraw();
        }
    });
}