use rayon::prelude::*;

use crate::draw::{ Bitmap, PixelBuf, PixelBufMut, FlatPixelBuf, FlatPixelBufMut };
use crate::math::{ Vec2, Rgba, Extent2 };

impl<I, B: PixelBuf> Bitmap<I, B>
{
    /// returns an iterator over the pixels in this bitmap
    ///
    /// ```
    /// for (pos, pixel) in frame.iter_pixels()
    /// {
    ///     if (*pixel[0] > 0)
    ///     {
    ///         println!("round some red!");
    ///     }
    /// }
    ///```
    pub fn iter_pixels(&self) -> impl Iterator<Item = (Vec2<i32>, &Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        (0..h)
            .flat_map(move |col| self.buf.row(col, w))
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }
}

impl<I, B: PixelBufMut> Bitmap<I, B>
{
    /// returns a mutable iterator over the pixels in this bitmap
    ///
    /// ```
    /// for (pos, pixel) in frame.iter_pixels_mut()
    /// {
    ///     // creates a black and white stripe pattern
    ///     if pos.x % 2 == 0
    ///     {
    ///         pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
    ///     }
    ///     else
    ///     {
    ///         pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
    ///     }
    /// }
    ///```
    pub fn iter_pixels_mut(&mut self) -> impl Iterator<Item = (Vec2<i32>, &mut Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        // borrow checker doesn't know that we won't be calling two buf.row_mut()
        // for the same column in the same lifetime
        let ptr = &*self as *const Self as *mut Self;

        (0..h)
            .flat_map(move |col| unsafe { &mut *ptr }.buf.row_mut(col, w))
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }
}

impl<I, B: FlatPixelBuf> Bitmap<I, B>
{
    /// returns an parallel iterator over the pixels in this bitmap
    ///
    /// ```
    /// frame.par_iter_pixels().for_each(|(pos, pixel)|
    /// {
    ///     if (*pixel[0] > 0)
    ///     {
    ///         println!("round some red!");
    ///     }
    /// });
    ///```
    pub fn par_iter_pixels(&self) -> impl ParallelIterator<Item = (Vec2<i32>, &Rgba<u8>)> + IndexedParallelIterator + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels()
            .par_iter()
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }
}

impl<I, B: FlatPixelBufMut> Bitmap<I, B>
{
    /// returns a parallel, mutable iterator over the pixels in this bitmap
    ///
    /// ```
    /// frame.par_iter_pixels_mut().for_each(|(pos, pixel)|
    /// {
    ///     // creates a black and white stripe pattern
    ///     if pos.x % 2 == 0
    ///     {
    ///         pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
    ///     }
    ///     else
    ///     {
    ///         pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
    ///     }
    /// });
    ///```
    pub fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = (Vec2<i32>, &mut Rgba<u8>)> + IndexedParallelIterator + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels_mut()
            .par_iter_mut()
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }
}

impl<I, B: PixelBuf> Bitmap<I, B>
{
    /// iterate non-overlapping "sub-bitmaps" or "chunks" in this bitmap,
    /// of size `size`. the remaining pixels, if any, are discarded from
    /// the iterator.
    ///
    /// this iterates over `Bitmap<Vec2<i32>, _>`, where each bitmap's `id()`
    /// is the position of its top-left corner in pixel-space.
    ///
    /// this method is comparable to a 2D version of [ChunksExact]
    ///
    /// [ChunksExact]: std::slice::ChunksExact
    pub fn iter_pixel_chunks(&self, size: Extent2<usize>) -> impl Iterator<Item = Bitmap<Vec2<i32>, Chunk<'_>>>
    {
        // iterate in row-by-row zig-zag pattern
        (0..self.height() / size.h)
        // do the cartesian product
        .flat_map(move |y| (0..self.width() / size.w).map(move |x| (x, y)))
        // once we have zig-zag indices, begin dividing chunks:
        .map(move |(x, y)|
        {
            // (x, y) is chunk index; remap to top-left corner in pixel
            // space
            let pos = Vec2::new(x * size.w, y * size.h);

            // create sparse 2D buffer(see `Chunks::buf` doc)
            let buf = (pos.y..pos.y + size.h)
                // go through each row in chunk
                .map(|y| &self.buf.row(y, self.width())[pos.x..pos.x + size.w])
                // collect to box(no other choice, chunk isn't contiguous)
                .collect::<Box<_>>();
            
            // convert pos
            let pos = pos.as_();
            
            // return chunks
            Bitmap::new(pos, Chunk(buf), size)
        })
    }

    pub fn iter_pixel_windows(&self, size: Extent2<usize>) -> impl Iterator<Item = Bitmap<Vec2<i32>, Chunk<'_>>>
    {
        // iterate in row-by-row zig-zag pattern
        (0..self.height() - size.h - 1)
        // do the cartesian product
        .flat_map(move |y| (0..self.width() - size.w - 1).map(move |x| (x, y)))
        // once we have zig-zag indices, begin dividing chunks:
        .map(move |(x, y)|
        {
            // create sparse 2D buffer(see `Chunks::buf` doc)
            let buf = (y..y + size.h)
                // go through each row in chunk
                .map(|y| &self.buf.row(y, self.width())[x..x + size.w])
                // collect to box(no other choice, chunk isn't contiguous)
                .collect::<Box<_>>();
            
            // convert pos
            let pos = Vec2::new(x as i32, y as i32);
            
            // return chunks
            Bitmap::new(pos, Chunk(buf), size)
        })
    }

    pub fn iter_pixel_overlapping_chunks(&self, size: Extent2<usize>, inc: Vec2<usize>) -> impl Iterator<Item = Bitmap<Vec2<i32>, Chunk<'_>>>
    {
        // iterate in row-by-row zig-zag pattern
        (0..(self.height() - size.h) / inc.y + 1)
        // do the cartesian product
        .flat_map(move |y| (0..(self.width() - size.w) / inc.x + 1).map(move |x| (x, y)))
        // once we have zig-zag indices, begin dividing chunks:
        .map(move |(x, y)|
        {
            // (x, y) is chunk index; remap to top-left corner in pixel
            // space
            let pos = Vec2::new(x * inc.x, y * inc.y);

            // create sparse 2D buffer(see `Chunks::buf` doc)
            let buf = (pos.y..pos.y + size.h)
                // go through each row in chunk
                .map(|y| &self.buf.row(y, self.width())[pos.x..pos.x + size.w])
                // collect to box(no other choice, chunk isn't contiguous)
                .collect::<Box<_>>();
            
            // convert pos
            let pos = pos.as_();
            
            // return chunks
            Bitmap::new(pos, Chunk(buf), size)
        })
    }
}

/// a single chunk in [Bitmap::iter_pixel_chunks] and
/// [Bitmap::iter_pixel_windows]
///
/// `self.0`:
/// outer-most array is columns, where
/// each element is a reference to a
/// row
/// ```text
/// buf: Box -> [&A, &B, &C]
///     &A -> [(0, 0), (1, 0), (2, 0)]
///     &B -> [(0, 1), (1, 1), (2, 1)]
///     &C -> [(0, 2), (1, 2), (2, 2)]
/// where &A, &B, and &C are assumed to be
/// in non-contiguous memory
/// ```
///
/// [Bitmap::iter_pixel_chunks]: super::Bitmap::iter_pixel_chunks
/// [Bitmap::iter_pixel_windows]: super::Bitmap::iter_pixel_windows
pub struct Chunk<'a>(Box<[&'a [Rgba<u8>]]>);

unsafe impl<'a> PixelBuf for Chunk<'a>
{
    fn raw_row<'b>(&'b self, col: usize, width: usize) -> &'b [u8]
    {
        use std::slice::from_raw_parts as slice;
        unsafe
        {
            slice(self.0[col].as_ptr() as *const u8, width * 4)
        }
    }

    fn try_raw_pixels<'b>(&'b self) -> Option<&'b [u8]>
    {
        None
    }

    fn row<'b>(&'b self, col: usize, _: usize) -> &'b [Rgba<u8>]
    {
        // override default implementation to avoid double
        // transmutes
        self.0[col]   
    }
}