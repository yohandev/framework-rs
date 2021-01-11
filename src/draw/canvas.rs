use std::ops::{Deref, DerefMut};

use crate::draw::Bitmap;

/// represents a framebuffer, which does everything a [Bitmap]
/// can while drawing to the screen and holding a reference to
/// a window(distinguishable with `Canvas::id()`)
///
/// [Bitmap]: crate::Bitmap
pub struct Canvas<'a> 
{
    /// pixel buffer frame
    pub(crate) inner: Bitmap<&'a mut [u8]>,
    /// unique identifier for the window this canvas is attached
    /// to
    pub(crate) id: CanvasId,
}

/// unique identifier for each [Canvas], and,
/// thereby each window
/// 
/// [Canvas]: crate::draw::Canvas
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanvasId(pub(crate) winit::window::WindowId);

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
    /// get this canvas' ID
    pub fn id(&self) -> CanvasId
    {
        self.id
    }
}