use crate::input::InputState;

/// represents the present state of the keyboard,
/// storing keys currently down/up and those pressed
/// and released exactly this update loop
#[derive(Debug, Clone, PartialEq)]
pub struct Keys
{
    /// key states where index is a keycode
    keys: [InputState; 255]
}

/// temporary keycode
pub type KeyCode = winit::event::VirtualKeyCode;

impl Keys
{
    /// is the key pressed this frame or held for the duration
    /// of this frame? useful for continuous input actions,
    /// such as character movement.
    #[inline]
    pub fn down(&self, key: KeyCode) -> bool
    {
        match self.keys[key as usize]
        {
            InputState::Down | InputState::Pressed => true,
            _ => false
        }
    }

    /// was the key up and then pressed exactly during this frame?
    /// useful for impulsive actions, like jumping.
    #[inline]
    pub fn pressed(&self, key: KeyCode) -> bool
    {
        self.keys[key as usize] == InputState::Pressed
    }

    /// was the key down and then released exactly during this
    /// frame?
    #[inline]
    pub fn released(&self, key: KeyCode) -> bool
    {
        self.keys[key as usize] == InputState::Released
    }
}

