use crate::Bitmap;

/// represents a framebuffer, which does everything a [Bitmap]
/// can while drawing to the screen
///
/// [Bitmap]: crate::Bitmap
pub type Frame<'a> = Bitmap<&'a mut [u8]>;