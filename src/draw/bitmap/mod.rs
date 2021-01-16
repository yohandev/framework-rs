mod bitmap2;
mod chunk;
mod buf;

pub use self::buf::{ PixelBuf, PixelBufMut, FlatPixelBuf, FlatPixelBufMut };
pub use self::bitmap2::Bitmap;
//pub use self::chunk::Chunk;

// use std::ops::{ Index, IndexMut };

// use rayon::prelude::*;

// use crate::math::*;

// /// represents a bitmap, which can be iterated and
// /// drawn to
// ///
// /// the second generic argument `B` is the inner storage
// /// for pixels(raw [u8]) and `I` is this bitmap's ID, if
// /// any. that ID should be `Copy` to access it later
// #[derive(Debug, Default, Clone)]
// pub struct Bitmap<I, B: Buf>
// {
//     /// this bitmap's ID
//     id: I,

//     /// inner byte array representing this bitmap
//     inner: B,
//     /// width and height, in pixels, of this bitmap
//     size: Extent2<usize>,

//     /// current stroke colour, if any
//     pub stroke: Option<Rgba<u8>>,
//     /// current fill colour, if any
//     pub fill: Option<Rgba<u8>>,
// }

// /// restrictions for a type that can be used as a bitmap
// /// pixel buffer
// pub trait Buf: AsRef<[u8]> + AsMut<[u8]> { }

// impl<I, B: Buf> Bitmap<I, B>
// {
//     /// create a new bitmap from its raw parts
//     pub fn new(id: I, inner: B, size: impl Into<Extent2<usize>>) -> Self
//     {
//         // convert
//         let size = size.into();

//         debug_assert_eq!(inner.as_ref().len() % 4, 0);
//         debug_assert_eq!(inner.as_ref().len() / 4, size.w * size.h);

//         // pen
//         let stroke = Some(Rgba::white());
//         let fill = Some(Rgba::grey(0x80));

//         Self { id, inner, size, stroke, fill }
//     }

//     /// get this bitmap' ID if it's used(otherwise
//     /// get returns an empty tuple)
//     #[inline]
//     pub fn id(&self) -> &I
//     {
//         &self.id
//     }

//     /// get this bitmap's width and height, in pixels
//     #[inline]
//     pub fn size(&self) -> Extent2<usize>
//     {
//         self.size
//     }

//     /// get this bitmap's width, in pixels
//     #[inline]
//     pub fn width(&self) -> usize
//     {
//         self.size.w
//     }

//     /// get this bitmap's height, in pixels
//     #[inline]
//     pub fn height(&self) -> usize
//     {
//         self.size.h
//     }

//     /// get this bitmap's area(width * height), in pixels
//     #[inline]
//     pub fn area(&self) -> usize
//     {
//         self.width() * self.height()
//     }

//     /// get the raw pixel bytes in this bitmap
//     ///
//     /// returns a slice of size width * height * 4
//     #[inline]
//     pub fn raw_pixels(&self) -> &[u8]
//     {
//         self.inner.as_ref()
//     }

//     /// get the raw pixel bytes in this bitmap, mutably
//     ///
//     /// returns a slice of size width * height * 4
//     #[inline]
//     pub fn raw_pixels_mut(&mut self) -> &mut [u8]
//     {
//         self.inner.as_mut()
//     }

//     /// get the pixels in this bitmap
//     ///
//     /// returns a slice of size width * height
//     #[inline]
//     pub fn pixels(&self) -> &[Rgba<u8>]
//     {
//         use std::slice::from_raw_parts as slice;
//         unsafe
//         {
//             slice(self.raw_pixels().as_ptr() as *const Rgba<u8>, self.area())
//         }
//     }

//     /// get the pixels in this bitmap, mutably
//     ///
//     /// returns a slice of size width * height
//     #[inline]
//     pub fn pixels_mut(&mut self) -> &mut [Rgba<u8>]
//     {
//         use std::slice::from_raw_parts_mut as slice;
//         unsafe
//         {
//             slice(self.raw_pixels_mut().as_ptr() as *mut Rgba<u8>, self.area())
//         }
//     }

//     /// returns an iterator over the pixels in this bitmap
//     ///
//     /// ```
//     /// for (pos, pixel) in frame.iter_pixels()
//     /// {
//     ///     if (*pixel[0] > 0)
//     ///     {
//     ///         println!("round some red!");
//     ///     }
//     /// }
//     ///```
//     pub fn iter_pixels(&self) -> impl Iterator<Item = (Vec2<i32>, &Rgba<u8>)> + '_
//     {
//         let w = self.width();
//         let h = self.height();

//         self.pixels()
//             .iter()
//             .enumerate()
//             .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
//     }

//     /// returns a mutable iterator over the pixels in this bitmap
//     ///
//     /// ```
//     /// for (pos, pixel) in frame.iter_pixels_mut()
//     /// {
//     ///     // creates a black and white stripe pattern
//     ///     if pos.x % 2 == 0
//     ///     {
//     ///         pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
//     ///     }
//     ///     else
//     ///     {
//     ///         pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
//     ///     }
//     /// }
//     ///```
//     pub fn iter_pixels_mut(&mut self) -> impl Iterator<Item = (Vec2<i32>, &mut Rgba<u8>)> + '_
//     {
//         let w = self.width();
//         let h = self.height();

//         self.pixels_mut()
//             .iter_mut()
//             .enumerate()
//             .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
//     }

//     /// returns an parallel iterator over the pixels in this bitmap
//     ///
//     /// ```
//     /// frame.par_iter_pixels().for_each(|(pos, pixel)|
//     /// {
//     ///     if (*pixel[0] > 0)
//     ///     {
//     ///         println!("round some red!");
//     ///     }
//     /// });
//     ///```
//     pub fn par_iter_pixels(&self) -> impl ParallelIterator<Item = (Vec2<i32>, &Rgba<u8>)> + '_
//     {
//         let w = self.width();
//         let h = self.height();

//         self.pixels()
//             .par_iter()
//             .enumerate()
//             .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
//     }

//     /// returns a parallel, mutable iterator over the pixels in this bitmap
//     ///
//     /// ```
//     /// frame.par_iter_pixels_mut().for_each(|(pos, pixel)|
//     /// {
//     ///     // creates a black and white stripe pattern
//     ///     if pos.x % 2 == 0
//     ///     {
//     ///         pixel.copy_from_slice(&[0xff, 0xff, 0xff, 0xff]);
//     ///     }
//     ///     else
//     ///     {
//     ///         pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
//     ///     }
//     /// });
//     ///```
//     pub fn par_iter_pixels_mut(&mut self) -> impl ParallelIterator<Item = (Vec2<i32>, &mut Rgba<u8>)> + '_
//     {
//         let w = self.width();
//         let h = self.height();

//         self.pixels_mut()
//             .par_iter_mut()
//             .enumerate()
//             .map(move |(i, px)| (Vec2::new((i % w) as i32, (i / h) as i32), px))
//     }

//     /// iterate non-overlapping "sub-bitmaps" or "chunks" in this bitmap,
//     /// of size `size`. the remaining pixels, if any, are discarded from
//     /// the iterator
//     ///
//     /// this method is comparable to a 2D version of [ChunksExact]
//     ///
//     /// [ChunksExact]: std::slice::ChunksExact
//     pub fn iter_pixel_chunks(&self, size: impl Into<Extent2<usize>>) -> impl Iterator<Item = super::Chunk<'_>>
//     {
//         self::chunk::iter_pixel_chunks(self, size.into())
//     }

//     /// set the fill colour to be used for any future drawing calls.
//     /// this is a shorthand for `canvas.fill = Some(col)`
//     #[inline]
//     pub fn fill(&mut self, col: Rgba<u8>)
//     {
//         self.fill = Some(col);
//     }

//     /// set the stroke colour to be used for any future drawing calls
//     /// this is a shorthand for `canvas.stroke = Some(col)`
//     #[inline]
//     pub fn stroke(&mut self, col: Rgba<u8>)
//     {
//         self.stroke = Some(col);
//     }

//     /// any future drawing calls will have no fill colour.
//     /// this is a shorthand for `canvas.fill = None`
//     #[inline]
//     pub fn no_fill(&mut self)
//     {
//         self.fill = None;
//     }

//     /// any future drawing calls will have no stroke colour.
//     /// this is a shorthand for `canvas.stroke = None`
//     #[inline]
//     pub fn no_stroke(&mut self)
//     {
//         self.stroke = None;
//     }

//     /// fills this entire bitmap with a color. this is much more efficient
//     /// than iterating through the pixels and individually setting their
//     /// colors.
//     pub fn background(&mut self, col: Rgba<u8>)
//     {
//         incremental_fill(self.pixels_mut(), col);
//     }

//     /// paste another bitmap on top of this one, clipping any invisible
//     /// pixels and (optionally) translating it
//     ///
//     /// the source bitmap isn't affected
//     pub fn image<U>(&mut self, src: &Bitmap<U, impl Buf>, pos: Vec2<i32>)
//     {
//         // givens
//         let dst_size: Vec2<i32> = self.size().as_::<i32>().into();
//         let src_size: Vec2<i32> = src.size().as_::<i32>().into();

//         let src_buf = src.pixels();
//         let dst_buf = self.pixels_mut();

//         // as you iterate src's pixels; [0, src_width] and [0, src_height]
//         let src_min = pos.map2(src_size, |p, s| (if p < 0 { -p } else { 0 }).min(s));
//         let src_max = pos.map3(src_size, dst_size, |p, ss, ds| if p + ss > ds { ds - p } else { ss });
  
//         // as you copy to dst's pixels; [0, dst_width] and [0, dst_height]
//         let dst_min_x = if pos.x < 0 { 0 } else { pos.x };
//         let dst_max_x = dst_min_x + (src_max.x - src_min.x);

//         // nothing to copy
//         if dst_max_x < dst_min_x
//         {
//             return;
//         }

//         // iterate vertically
//         for y in src_min.y..src_max.y
//         {
//             let src_str = (y * src_size.x + src_min.x) as usize;
//             let src_end = (y * src_size.x + src_max.x) as usize;

//             let dst_str = ((y + pos.y) * dst_size.x + dst_min_x) as usize;
//             let dst_end = ((y + pos.y) * dst_size.x + dst_max_x) as usize;

//             // copy entire horizontal segments at once
//             dst_buf[dst_str..dst_end].copy_from_slice(&src_buf[src_str..src_end]);
//         }
//     }

//     /// draws a line from `a` to `b`, clipping any pixels out of
//     /// bounds
//     pub fn line(&mut self, a: Vec2<i32>, b: Vec2<i32>)
//     {
//         use crate::util::Bresenham;

//         // stroke
//         if let Some(stroke) = self.stroke
//         {
//             for pos in Bresenham::new_bounded(a, b, self.size().as_())
//             {
//                 self[pos] = stroke;
//             }
//         }
//     }

//     /// draws a triangle with vertices `a`, `b`, and `c`, clipping
//     /// any pixels out of bounds
//     pub fn triangle(&mut self, a: Vec2<i32>, b: Vec2<i32>, c: Vec2<i32>)
//     {
//         use crate::util::Triangle;

//         // fill
//         if let Some(fill) = self.fill
//         {
//             for (pos, _) in Triangle::new_bounded([a, b, c], self.size().as_())
//             {
//                 self[pos] = fill;
//             }
//         }

//         // stroke
//         self.line(a, b);
//         self.line(b, c);
//         self.line(c, a);
//     }

//     /// draws a rectangle with top-left corner at `pos` and of
//     /// (width, height) `siz`. clips any pixels out of bounds.
//     pub fn rect(&mut self, pos: Vec2<i32>, siz: Vec2<i32>)
//     {
//         // size of this bitmap
//         let bounds: Vec2<i32> = self.size.as_().into();

//         // crop with top left corner(0, 0)
//         let siz = siz.map2(pos, |s, p| if p < 0 { s + p } else { s });
//         let pos = pos.map(|n| n.max(0));
//         // crop with bottom right corner(width - 1, height - 1)
//         let siz = siz.map3(pos, bounds, |siz, pos, bound|
//         {
//             let d = bound - (pos + siz);
//             if d < 0 { siz + d } else { siz }
//         });
        
//         // empty rectangle
//         if siz.x <= 0 || siz.y <= 0
//         {
//             return;
//         }

//         // fill
//         if let Some(fill) = self.fill
//         {
//             // first line indices
//             let first_ln_i = (pos.y * bounds.x + pos.x) as usize;
//             let offset = first_ln_i + siz.x as usize;

//             // split
//             let (src, dst) = self.pixels_mut().split_at_mut(offset);

//             // draw first line
//             let src = &mut src[first_ln_i..];
//             incremental_fill(src, fill);

//             // draw other lines by copying first line
//             for y in pos.y + 1..pos.y + siz.y
//             {
//                 let i = (y * bounds.x + pos.x) as usize - offset;
//                 let j = i + siz.x as usize;

//                 dst[i..j].copy_from_slice(src);
//             }
//         }
//         // stroke
//         if self.stroke.is_some()
//         {
//             let top_l = pos;
//             let btm_l = pos + Vec2::new(0, siz.y);
//             let top_r = pos + Vec2::new(siz.x, 0);
//             let btm_r = pos + siz;

//             self.line(top_l, top_r);
//             self.line(btm_l, btm_r);
//             self.line(top_l, btm_l);
//             self.line(top_r, btm_r);
//         }
//     }
// }

// /// blanket implementation
// impl<B: AsRef<[u8]> + AsMut<[u8]>> Buf for B { }

// impl<I, B: Buf> Index<Vec2<i32>> for Bitmap<I, B>
// {
//     type Output = Rgba<u8>;

//     /// get the pixel color at the given position in pixels. panics if
//     /// out of bound
//     #[inline]
//     fn index(&self, pos: Vec2<i32>) -> &Self::Output
//     {
//         // index
//         let ind = pos.y as usize * self.width() + pos.x as usize;

//         // get
//         &self.pixels()[ind]
//     }
// }

// impl<I, B: Buf> IndexMut<Vec2<i32>> for Bitmap<I,B>
// {
//     /// get the pixel color at the given position in pixels. panics if
//     /// out of bound
//     #[inline]
//     fn index_mut(&mut self, pos: Vec2<i32>) -> &mut Self::Output
//     {
//         // index
//         let ind = pos.y as usize * self.width() + pos.x as usize;

//         // get
//         &mut self.pixels_mut()[ind]
//     }
// }

/// incrementally fill a slice `buf` with `ele`
/// using a progressively larger memcpy...
/// 
/// works setting buf[0], then copying that to
/// buf[0..2], then copying buf[0..2] to buf[2..4],
/// then buf[0..4] to buf[4..8], etc.
fn incremental_fill<T: Copy>(buf: &mut [T], ele: T)
{
    // set the first element
    buf[0] = ele;

    // size is how much has been done so far(also the cursor)
    // rem is how much to be populated and may go below zero
    let mut siz = 1;
    let mut rem = buf.len() as isize;

    while rem > 0
    {
        // split what's already been cleared and what's remaining
        let (src, dst) = buf.split_at_mut(siz);

        // upper-bound index to copy
        let cpy = dst.len().min(siz);

        // copy over to clear some more
        dst[0..cpy].copy_from_slice(&src[0..cpy]);

        // grow by a factor of 2
        rem -= siz as isize;
        siz *= 2;
    }
}