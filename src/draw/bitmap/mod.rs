mod draw;
mod iter;
mod buf;

pub use self::buf::{ PixelBuf, PixelBufMut, FlatPixelBuf, FlatPixelBufMut };
pub use self::iter::Chunk;

use crate::math::{ Vec2, Rgba, Extent2 };

// represents a bitmap, which can be iterated and
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

use std::ops::{ Index, IndexMut };

impl<I, B: PixelBuf> Index<Vec2<i32>> for Bitmap<I, B>
{
    type Output = Rgba<u8>;

    /// get the pixel color at the given position in pixels. panics if
    /// out of bound
    #[inline]
    fn index(&self, pos: Vec2<i32>) -> &Self::Output
    {
        &self.buf.row(pos.y as usize, self.width())[pos.x as usize]
    }
}

impl<I, B: PixelBufMut> IndexMut<Vec2<i32>> for Bitmap<I,B>
{
    /// get the pixel color at the given position in pixels. panics if
    /// out of bound
    #[inline]
    fn index_mut(&mut self, pos: Vec2<i32>) -> &mut Self::Output
    {
        &mut self.buf.row_mut(pos.y as usize, self.width())[pos.x as usize]
    }
}