use crate::draw::Bitmap;

/// represents a framebuffer, which does everything a [Bitmap]
/// can while drawing to the screen
///
/// [Bitmap]: crate::Bitmap
pub type Canvas<'a> = Bitmap<&'a mut [u8]>;

/// unique identifier for each [Canvas], and,
/// thereby each window
/// 
/// [Canvas]: crate::draw::Canvas
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanvasId(winit::window::WindowId);