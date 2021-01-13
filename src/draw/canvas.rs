use std::ops::{ Deref, DerefMut };

use winit::window::Window as WinitWindow;

use crate::draw::Bitmap;

/// represents a framebuffer, which does everything a [Bitmap]
/// can while drawing to the screen and holding a reference to
/// a window(distinguishable with `Canvas::id()`)
///
/// [Bitmap]: crate::Bitmap
pub struct Canvas<'a> 
{
    /// pixel buffer frame
    pub(super) inner: Bitmap<&'a mut [u8]>,
    /// the window this canvas is attached to
    pub(super) window: &'a WinitWindow,
    /// this canvas' ID
    pub(super) id: CanvasId,
}

/// unique identifier for each [Canvas], and,
/// thereby each window
/// 
/// [Canvas]: crate::draw::Canvas
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanvasId(i32);

impl<'a> Deref for Canvas<'a>
{
    type Target = Bitmap<&'a mut [u8]>;

    fn deref(&self) -> &Self::Target
    {
        &self.inner
    }
}

impl<'a> DerefMut for Canvas<'a>
{
    fn deref_mut(&mut self) -> &mut Self::Target
    {
        &mut self.inner
    }
}

impl<'a> Canvas<'a>
{
    /// get this canvas' unique ID
    #[inline]
    pub fn id(&self) -> CanvasId
    {
        self.id
    }

    /// get the winit window this canvas is rendered to
    pub fn window(&self) -> &WinitWindow
    {
        self.window
    }
}

impl CanvasId
{
    /// the first canvas id
    pub(crate) fn zero() -> Self
    {
        Self(0)
    }

    /// get the next CanvasId(`self + 1`)
    pub(crate) fn next(self) -> Self
    {
        Self(self.0 + 1)
    }
}