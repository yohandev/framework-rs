use pixels::{ Pixels, SurfaceTexture };

use vek::Extent2;
use winit_input_helper::WinitInputHelper;

use winit::event_loop::{ ControlFlow, EventLoop, EventLoopWindowTarget };
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use winit::event::Event;

use crate::Sketch;

/// run the sketch, hyjacking the main thread until the
/// window is closed
pub fn run<T: Sketch>()
{
    let size = LogicalSize::new(T::SIZE.w as f64, T::SIZE.h as f64);

    let events = EventLoop::new();
    let window = WindowBuilder::new()
        .with_min_inner_size(size)
        .with_inner_size(size)
        .with_title(T::TITLE)
        .build(&events)
        .unwrap();

    let mut pixels =
    {
        let w_size = window.inner_size();
        let surf_tex = SurfaceTexture::new(w_size.width, w_size.height, &window);
        
        Pixels::new(T::SIZE.w as u32, T::SIZE.h as u32, surf_tex).unwrap()
    };

    let mut input = WinitInputHelper::new();
    let mut time = Time::new();
    
    // start state
    state.start();

    events.run(move |evt, _, control_flow|
    {
        // draw the current frame
        if let Event::RedrawRequested(_) = evt
        {
            // get the frame
            let mut frame = Frame::new(pixels.get_frame(), T::SIZE);
            
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

/// creates a new window
fn create_window(evt: &EventLoopWindowTarget<()>, size: Extent2<u32>, title: impl Into<String>) -> (Window, Pixels<Window>)
{
    let size_f64 = LogicalSize::new(size.w as f64, size.h as f64);

    let window = WindowBuilder::new()
        .with_min_inner_size(size_f64)
        .with_inner_size(size_f64)
        .with_title(title)
        .build(&evt)
        .unwrap();

    let mut pixels =
    {
        let w_size = window.inner_size();
        let surf_tex = SurfaceTexture::new(w_size.width, w_size.height, &window);
        
        Pixels::new(size.w, size.h, surf_tex).unwrap()
    };
}