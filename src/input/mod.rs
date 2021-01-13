mod mouse;
mod keys;

pub use self::mouse::Mouse;
pub use self::keys::Keys;

use winit::window::WindowId;
use winit::event::Event;

/// stores raw keyboard, mouse, and [TODO] controller input.
/// It caches physical buttons that are held or up, as well as
/// buttons pressed or released during the duration of this frame.
#[derive(Debug)]
pub struct Input
{
    /// mouse input
    mouse: Mouse,
    /// keyboard input
    keys: Keys,
}

/// enumeration to cache the state of input keys and buttons
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
enum InputState
{
    /// input is not held for the duration of this frame
    Up = 0,
    /// input is held for the duration of this frame
    Down = 1,
    /// input started being pressed during this frame
    Pressed = 2,
    /// input stopped being pressed during this frame
    Released = 3,
}

/// yield of `Input::process()` relevant to the `framework::run`
/// function
pub(crate) enum ProcessedEvent
{
    /// the given window was resized to the size
    WindowResized(WindowId, (u32, u32)),
    /// the given window was requested to close
    WindowClose(WindowId),
    /// the given window should render and refresh
    ShouldRender(WindowId),
    /// the framework should call `Sketch::update()`
    ShouldUpdate,
    /// nothing to be reported
    None,
}

impl Input
{
    /// create a new `Input` instance
    pub(crate) fn new() -> Self
    {
        Self
        {
            mouse: Mouse::new(),
            keys: Keys::new(),
        }
    }

    /// process a raw incoming winit event
    pub(crate) fn process(&mut self, event: Event<()>) -> ProcessedEvent
    {
        use winit::event::WindowEvent;

        match event
        {
            Event::NewEvents(_) =>
            {
                // reset mouse and keyboard
                self.mouse.reset();
                self.keys.reset();

                ProcessedEvent::None
            }
            Event::WindowEvent { window_id, event } =>
            {
                // process mouse and keyboard
                self.mouse.process(&event);
                self.keys.process(&event);

                // process additional
                match event
                {
                    WindowEvent::Resized(siz) =>
                    {
                        ProcessedEvent::WindowResized(window_id, (siz.width, siz.height))
                    }
                    WindowEvent::CloseRequested =>
                    {
                        ProcessedEvent::WindowClose(window_id)
                    }
                    WindowEvent::DroppedFile(_) =>
                    {
                        // TODO: store dropped file

                        ProcessedEvent::None
                    }
                    WindowEvent::Focused(_) =>
                    {
                        // TODO: keep track of focus

                        ProcessedEvent::None
                    },
                    _ => ProcessedEvent::None
                }
            }
            Event::MainEventsCleared => ProcessedEvent::ShouldUpdate,
            Event::RedrawRequested(window_id) => ProcessedEvent::ShouldRender(window_id),
            _ => ProcessedEvent::None
        }
    }
}