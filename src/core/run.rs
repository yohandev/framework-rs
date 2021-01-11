use pixels::{ Pixels, SurfaceTexture };

use winit_input_helper::WinitInputHelper;

use winit::event_loop::{ ControlFlow, EventLoop };
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::Event;

use crate::core::{ Sketch, Time, window::Window };
use crate::draw::Canvas;

/// run the sketch, hyjacking the main thread until the
/// window is closed
pub fn run<T: Sketch>()
{
    // create event loop
    let events = EventLoop::new();

    // inputs
    let mut input = WinitInputHelper::new();
    let mut time = Time::new();
    
    // create state
    let mut state = T::setup(app);

    events.run(move |evt, _, control_flow|
    {
        // draw the current frame
        if let Event::RedrawRequested(_) = evt
        {
            // get the frame
            let mut frame = Canvas::new(pixels.get_frame(), T::SIZE);
            
            // update buffer
            state.draw(&mut frame);

            // render
            if pixels.render().is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // handle input events
        if input.update(&evt)
        {
            // close events
            if input.quit()
            {
                *control_flow = ControlFlow::Exit;
                return;
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