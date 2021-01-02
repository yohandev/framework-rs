mod bitmap;
mod frame;
mod math2;
mod time;
mod img;
mod app;

pub mod util;

pub use bitmap::Bitmap;
pub use frame::Frame;
pub use time::Time;
pub use img::Image;
pub use app::App;

pub use rayon::prelude::ParallelIterator;
pub use vek as math;

/// run the app, hyjacking the main thread until the
/// window is closed
pub fn run<T: App + Default>()
{
    run_instance(T::default());
}

/// run the app, hyjacking the main thread until the
/// window is closed
///
/// takes an instance of an app; use [run] if your app type
/// implements [Default]
///
/// [run]: crate::run
/// [Default]: Default
pub fn run_instance<T: App>(mut state: T)
{
    use pixels::{ Pixels, SurfaceTexture };

    use winit::dpi::LogicalSize;
    use winit::event::Event;
    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::window::WindowBuilder;
    use winit_input_helper::WinitInputHelper;

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
    
    events.run(move |evt, _, control_flow|
    {
        // Draw the current frame
        if let Event::RedrawRequested(_) = evt
        {
            // get the frame
            let mut frame = Frame::new(pixels.get_frame(), T::SIZE);
            
            // update buffer
            state.render(&mut frame);

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