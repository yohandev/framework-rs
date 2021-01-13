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

