use crate::draw::CanvasId;
use crate::math::Extent2;
use crate::input::Input;
use crate::core::Time;

/// handle to the application running this `Sketch`.
/// this is how `Canvas`es are created or input and
/// time is accessed.
///
/// internally, the `App` struct is hollow; it holds
/// only references to its composing components(time,
/// input, etc.) which `framework::run` feeds to it
/// whenever needed. this doesn't change much on the
/// user end, other than `App` not being `'static`
pub struct App<'a>
{
    pub(super) input: &'a Input,
    pub(super) time: &'a Time,

    pub(super) requests: Vec<AppRequest>
}

/// queued instructions for an `App` that are then
/// polled by `framework::run`
pub(super) enum AppRequest
{
    /// create a new window/canvas
    CreateCanvas(String, Extent2<usize>),
    // close a canvas
    CloseCanvas(CanvasId),
}