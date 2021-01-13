use crate::input::{ Input, Mouse, Keys };
use crate::draw::CanvasId;
use crate::math::Extent2;
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

impl<'a> App<'a>
{
    /// get the current `Time` information for this
    /// frame
    #[inline]
    pub fn time(&self) -> &Time
    {
        self.time
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
        self.requests.push(AppRequest::CreateCanvas(title.into(), size.into()));
    }

    /// destroys a `Canvas` given its ID if it exists, or
    /// does nothing if it doesn't
    pub fn destroy_canvas(&mut self, id: CanvasId)
    {
        self.requests.push(AppRequest::CloseCanvas(id));
    }
}