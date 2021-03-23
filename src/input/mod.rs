mod btn;
mod mouse;
mod keys;
mod time;

pub use self::mouse::{ Mouse, MouseButton };
pub use self::keys::{ Keys, KeyCode };
pub use self::time::Time;

use crate::app::Windows;

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

    /// with artifical frame rate limiting, some events can be
    /// missed. this flag, when set to true, will clear any input
    /// indicating that a frame has passed and inputs haven't
    /// been missed
    pub(super) reset: bool,
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

            reset: false,
        }
    }

    /// process a raw incoming winit event
    pub(crate) fn process(&mut self, event: &Event<()>, windows: &mut Windows) -> ProcessedEvent
    {
        use winit::event::WindowEvent;

        match event
        {
            Event::NewEvents(_) =>
            {
                // reset mouse and keyboard
                if self.reset
                {
                    self.mouse.reset();
                    self.keys.reset();

                    self.reset = false;
                }

                ProcessedEvent::None
            }
            Event::WindowEvent { window_id, event: evt } =>
            {
                // fetch window this event refers to
                //
                // if window doesn't exist, we don't care do update
                // its mouse/keyboard state
                if let Some(window) = windows.get_mut(&window_id)
                {
                    // process mouse and keyboard
                    self.mouse.process(&evt, window);
                    self.keys.process(&evt);

                    // let egui process raw event
                    window.gui.process(&event)
                }
                
                // process additional
                match evt
                {
                    WindowEvent::Resized(siz) =>
                    {
                        ProcessedEvent::WindowResized(*window_id, (siz.width, siz.height))
                    }
                    WindowEvent::CloseRequested =>
                    {
                        ProcessedEvent::WindowClose(*window_id)
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
            Event::RedrawRequested(window_id) => ProcessedEvent::ShouldRender(*window_id),
            _ => ProcessedEvent::None
        }
    }

    /// get `Keys` from `self`
    #[inline]
    pub fn keys(&self) -> &Keys
    {
        &self.keys
    }

    /// get `Mouse` from `self`
    #[inline]
    pub fn mouse(&self) -> &Mouse
    {
        &self.mouse
    }
}