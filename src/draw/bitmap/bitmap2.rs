use crate::draw::{ PixelBuf, PixelBufMut, FlatPixelBuf, FlatPixelBufMut };
use crate::math::{ Vec2, Rgba, Extent2 };

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