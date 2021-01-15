use crate::draw::Bitmap;

/// represents a framebuffer, which does everything a [Bitmap]
/// can while drawing to the screen and holding a reference to
/// a window(distinguishable with `Canvas::id()`)
///
/// [Bitmap]: crate::Bitmap
pub type Canvas<'a> = Bitmap<CanvasId, &'a mut [u8]>;

/// unique identifier for each [Canvas], and,
/// thereby each window
/// 
/// [Canvas]: crate::draw::Canvas
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanvasId(i32);

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