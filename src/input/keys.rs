use winit::event::WindowEvent;

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
    /// reset the input states within `self` to prepare for
    /// a new frame
    pub(super) fn reset(&mut self)
    {
        for key in self.keys.iter_mut()
        {
            *key = match *key
            {
                InputState::Pressed => InputState::Down,
                InputState::Released => InputState::Up,
                InputState::Down => InputState::Down,
                InputState::Up => InputState::Up,
            };
        }
    }

    /// update the internal input states within `self` using
    /// the relevant winit-provided event
    pub(super) fn update(&mut self, event: &WindowEvent)
    {
        use winit::event::{ KeyboardInput, ElementState };

        // yay for destructuring!
        if let WindowEvent::KeyboardInput { input: KeyboardInput { virtual_keycode: Some(code), state, .. }, .. } = event
        {
            // convert to index
            let code = *code as usize;

            self.keys[code] = match (state, self.keys[code])
            {
                (ElementState::Pressed, InputState::Up) => InputState::Pressed,
                (ElementState::Released, InputState::Down) => InputState::Released,
                (ElementState::Pressed, _) => InputState::Down,
                (ElementState::Released, _) => InputState::Up,
            };
        }
    }

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

