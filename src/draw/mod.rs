// pub mod macros;

mod canvas;
mod bitmap;
mod window;
mod image;

pub use self::canvas::{ Canvas, CanvasId };
pub use self::bitmap::{ Bitmap, Buf, Chunk, PixelBuf, PixelBufMut, FlatPixelBuf, FlatPixelBufMut };
pub use self::image::Image;

pub(crate) use self::window::Window;

pub use rayon::prelude::ParallelIterator;