use std::collections::HashMap;

use winit::event_loop::{ ControlFlow, EventLoopWindowTarget as WindowTarget };
use winit::window::WindowId;
use winit::event::Event;

use crate::input::{ Input, Mouse, Keys, Time, ProcessedEvent };
use crate::draw::{ CanvasId, Window };
use crate::math::Extent2;
use crate::Sketch;

/// handle to the application running this `Sketch`.
/// this is how `Canvas`es are created or input and
/// time is accessed.
pub struct App
{
    /// input state
    pub(super) input: Input,
    /// time state
    pub(super) time: Time,

    /// windows currently open
    pub(super) windows: HashMap::<WindowId, Window>,
    /// window creation requests
    pub(super) requests: Vec<(String, Extent2<usize>)>
}

impl App
{
    /// create a new app
    pub(super) fn new() -> Self
    {
        Self
        {
            input: Input::new(),
            time: Time::new(),
            windows: HashMap::new(),
            requests: Vec::new(),
        }
    }

    /// process an incoming winit event and interact with
    /// `Sketch`, inner `Time`, `Input`, and windows accordingly.
    pub(super) fn process_event(
        &mut self,
        sketch: &mut impl Sketch,
        event: Event<()>,
        window_target: &WindowTarget<()>,
        control_flow: &mut ControlFlow
    )
    {
        match self.input.process(event)
        {
            // notify `Pixels` of the window resize
            ProcessedEvent::WindowResized(id, (w, h)) =>
            {
                if let Some(window) = self.windows.get_mut(&id)
                {
                    window.pixels.resize(w, h);
                }
            }
            // close window and quit if needed
            ProcessedEvent::WindowClose(id) =>
            {
                // stop keeping track of and drop window
                self.windows.remove(&id);

                // quit?
                if self.windows.is_empty()
                {
                    return *control_flow = ControlFlow::Exit;
                }
            }
            // render to the given window
            ProcessedEvent::ShouldRender(id) =>
            {
                // get the frame
                if let Some(window) = self.windows.get_mut(&id)
                {
                    // update buffer
                    sketch.draw(&mut window.get_frame());

                    // render
                    if window.pixels.render().is_err()
                    {
                        return *control_flow = ControlFlow::Exit;
                    }
                }
            }
            // update the Sketch state and request new events
            ProcessedEvent::ShouldUpdate =>
            {
                // update time
                self.time.update();

                // update state
                sketch.update(self);

                // process requests
                self.process_requests(window_target);
                
                // request a redraw on each window
                for (_, window) in self.windows.iter_mut()
                {
                    window.winit.request_redraw();
                }
            }
            ProcessedEvent::None => { }
        }
    }

    /// process window creation requests. this should be
    /// called right after the `App` has been mutuably passed
    /// to the `Sketch`
    pub(super) fn process_requests(&mut self, target: &WindowTarget<()>)
    {
        while let Some((title, size)) = self.requests.pop()
        {
            let window = Window::new(target, title, size);

            self.windows.insert(window.winit.id(), window);
        }
    }

    /// get the current `Time` information for this
    /// frame
    #[inline]
    pub fn time(&self) -> &Time
    {
        &self.time
    }

    /// get the current state of the keyboard for this
    /// frame
    #[inline]
    pub fn keys(&self) -> &Keys
    {
        self.input.keys()
    }

    /// get the current state of the mouse for this
    /// frame
    #[inline]
    pub fn mouse(&self) -> &Mouse
    {
        self.input.mouse()
    }

    /// create a new `Canvas`, and effectively a window, to
    /// draw to
    pub fn create_canvas(&mut self, title: impl Into<String>, size: impl Into<Extent2<usize>>)
    {
        self.requests.push((title.into(), size.into()));
    }

    /// destroys a `Canvas` given its ID if it exists, or
    /// does nothing if it doesn't
    pub fn destroy_canvas(&mut self, id: CanvasId)
    {
        self.windows.remove(&id.0);
    }
}