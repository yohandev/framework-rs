use rayon::prelude::*;

use crate::draw::{ PixelBuf, PixelBufMut, FlatPixelBuf, FlatPixelBufMut };
use crate::math::{ Vec2, Rgba, Extent2 };

use super::incremental_fill;

/// represents a bitmap, which can be iterated and
/// drawn to
///
/// the second generic argument `B` is the inner storage
/// for pixels(raw [u8]) and `I` is this bitmap's ID, if
/// any. that ID should be `Copy` to access it later
#[derive(Debug, Default, Clone)]
pub struct Bitmap<I, B>
{
    /// this bitmap's ID
    id: I,

    /// inner byte array representing this bitmap
    buf: B,
    /// width and height, in pixels, of this bitmap
    size: Extent2<usize>,

    /// current stroke colour, if any
    pub stroke: Option<Rgba<u8>>,
    /// current fill colour, if any
    pub fill: Option<Rgba<u8>>,
}

impl<I, B> Bitmap<I, B>
{
    /// create a new bitmap from its raw parts
    pub fn new(id: I, buf: B, size: Extent2<usize>) -> Self
    {
        // pen
        let stroke = Some(Rgba::white());
        let fill = Some(Rgba::grey(0x80));

        Self { id, buf, size, stroke, fill }
    }

    /// get this bitmap' ID if it's used(otherwise
    /// get returns an empty tuple)
    #[inline]
    pub fn id(&self) -> &I
    {
        &self.id
    }

    /// get this bitmap's width and height, in pixels
    #[inline]
    pub fn size(&self) -> Extent2<usize>
    {
        self.size
    }

    /// get this bitmap's width, in pixels
    #[inline]
    pub fn width(&self) -> usize
    {
        self.size.w
    }

    /// get this bitmap's height, in pixels
    #[inline]
    pub fn height(&self) -> usize
    {
        self.size.h
    }

    /// get this bitmap's area(width * height), in pixels
    #[inline]
    pub fn area(&self) -> usize
    {
        self.width() * self.height()
    }

    /// set the fill colour to be used for any future drawing calls.
    /// this is a shorthand for `canvas.fill = Some(col)`
    #[inline]
    pub fn fill(&mut self, col: Rgba<u8>)
    {
        self.fill = Some(col);
    }

    /// set the stroke colour to be used for any future drawing calls
    /// this is a shorthand for `canvas.stroke = Some(col)`
    #[inline]
    pub fn stroke(&mut self, col: Rgba<u8>)
    {
        self.stroke = Some(col);
    }

    /// any future drawing calls will have no fill colour.
    /// this is a shorthand for `canvas.fill = None`
    #[inline]
    pub fn no_fill(&mut self)
    {
        self.fill = None;
    }

    /// any future drawing calls will have no stroke colour.
    /// this is a shorthand for `canvas.stroke = None`
    #[inline]
    pub fn no_stroke(&mut self)
    {
        self.stroke = None;
    }
}

impl<I, B: FlatPixelBuf> Bitmap<I, B>
{
    /// get the pixels in this bitmap
    ///
    /// returns a slice of size width * height
    #[inline]
    pub fn pixels(&self) -> &[Rgba<u8>]
    {
        self.buf.pixels()
    }
}

impl<I, B: FlatPixelBufMut> Bitmap<I, B>
{
    /// get the pixels in this bitmap, mutably
    ///
    /// returns a slice of size width * height
    #[inline]
    pub fn pixels_mut(&mut self) -> &mut [Rgba<u8>]
    {
        self.buf.pixels_mut()
    }
}

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
    pub fn par_iter_pixels(&self) -> impl ParallelIterator<Item = (Vec2<i32>, &Rgba<u8>)> + '_
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
    pub fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = (Vec2<i32>, &mut Rgba<u8>)> + '_
    {
        let w = self.width();
        let h = self.height();

        self.pixels_mut()
            .par_iter_mut()
            .enumerate()
            .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
    }
}

impl<I, B: PixelBufMut> Bitmap<I, B>
{
    /// fills this entire bitmap with a color. this is much more efficient
    /// than iterating through the pixels and individually setting their
    /// colors.
    pub fn background(&mut self, col: Rgba<u8>)
    {
        // most optimized, incremental fill method:
        if let Some(buf) = self.buf.try_pixels_mut()
        {
            super::incremental_fill(buf, col);
        }
        // less optimized, row-by-row fill method:
        else
        {
            // get the first row
            let row0 = 
            {
                // SAFETY: borrow checker isn't smart enough to know
                // we're only borrowing the first row once
                let this = &*self as *const Self as *mut Self;

                unsafe { &mut *this }.buf.row_mut(0, self.width())
            };

            // fill the first row
            super::incremental_fill(row0, col);

            // fill every other row
            for y in 1..self.height()
            {
                // get the row
                let row = self.buf.row_mut(y, self.width());

                // copy the first row into this one
                row.copy_from_slice(row0);
            }
        }
    }
}

// // incrementally fill a slice `buf` with `ele`
// /// using a progressively larger memcpy...
// /// 
// /// works setting buf[0], then copying that to
// /// buf[0..2], then copying buf[0..2] to buf[2..4],
// /// then buf[0..4] to buf[4..8], etc.
// fn incremental_fill<T: Copy>(buf: &mut [T], ele: T)
// {
//     // set the first element
//     buf[0] = ele;

//     // size is how much has been done so far(also the cursor)
//     // rem is how much to be populated and may go below zero
//     let mut siz = 1;
//     let mut rem = buf.len() as isize;

//     while rem > 0
//     {
//         // split what's already been cleared and what's remaining
//         let (src, dst) = buf.split_at_mut(siz);

//         // upper-bound index to copy
//         let cpy = dst.len().min(siz);

//         // copy over to clear some more
//         dst[0..cpy].copy_from_slice(&src[0..cpy]);

//         // grow by a factor of 2
//         rem -= siz as isize;
//         siz *= 2;
//     }
// }