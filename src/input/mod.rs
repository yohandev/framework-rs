mod mouse;
mod keys;

pub use self::mouse::Mouse;
pub use self::keys::Keys;

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

// TODO use own types for keycode and mouse button
pub use winit::event::{ Event };

/// stores raw keyboard, mouse, and [TODO] controller input.
/// It caches physical buttons that are held or up, as well as
/// buttons pressed or released during the duration of this frame.
pub struct Input
{
    /// mouse input
    mouse: Mouse,
    /// keyboard input
    keys: Keys,
}

impl Input
{
    pub(crate) fn update(&mut self, evt: Event<()>)
    {
        if let Event::NewEvents(_) = evt
        {
            self.mouse.reset();
            self.keys.reset();
        }
        if let Event::WindowEvent { event: window_event, ..} = evt
        {
            self.mouse.update(&window_event);
            self.keys.update(&window_event);
        }
    }
}