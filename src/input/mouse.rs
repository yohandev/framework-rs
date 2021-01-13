use crate::input::InputState;
use crate::math::Vec2;

/// represents the present state of the mouse,
/// storing buttons currently down/up and those
/// pressed and released exactly this update loop.
/// also stores current mouse position/delta and
/// scroll wheel info.
#[derive(Debug, Clone, PartialEq)]
pub struct Mouse
{
    /// current mouse position
    pos: Vec2<f64>,
    /// delta mouse position
    del: Vec2<f64>,

    /// mouse button states where index is a mouse button
    btn: [InputState; 255],

    /// scroll wheel
    scroll: Vec2<f32>,
}