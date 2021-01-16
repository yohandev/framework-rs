use crate::math::Rgba;

/// type that can be used as a bitmap pixel buffer
///
/// while the entire buffer doesn't have to be in
/// contiguous memory, the only requirement is that
/// each row of pixels *is* in contiguous memory.
///
/// note: implementors of `PixelBuf` should *never*
/// grow or shrink
pub unsafe trait PixelBuf
{
    /// get the raw pixels at the given column
    ///
    /// ie. for an image like so:
    /// ```text
    /// ⎡ A, B, C ⎤
    /// | D, E, F |
    /// ⎣ G, H, I ⎦
    /// ```
    /// ...`raw_row(0)` would return `[A, B, C]`, while
    /// `raw_row(3)` would panic. (in `u8` representations
    /// of course)
    ///
    /// the `PixelBuf` implementor isn't expected to keep
    /// track of its own size, so the promised `width` of
    /// the bitmap is passed in this method. note that `width`
    /// is in pixels(`Rgba<u8>`), NOT `u8`s.
    ///
    /// safety:
    /// 1. should panic if `col` is out of bounds
    /// 2. the slice returned should always be of constant
    ///    length equal to `width`, no matter what value
    ///    `col` is and assuming condition (1) isn't violated.
    /// 3. the slice returned must be safely transmutable
    ///    to `Rgba<u8>`
    /// 4. calling `raw_row` or `row` for any two different values
    ///    `col` should *never* return a slice that points to the
    ///    same memory!
    fn raw_row<'a>(&'a self, col: usize, width: usize) -> &'a [u8];

    /// get the pixels at the given column. panics if `col`
    /// is out of bounds
    ///
    /// the `PixelBuf` implementor isn't expected to keep
    /// track of its own size, so the promised `width` of
    /// the bitmap is passed in this method.
    #[inline]
    fn row<'a>(&'a self, col: usize, width: usize) -> &'a [Rgba<u8>]
    {
        use std::slice::from_raw_parts as slice;
        unsafe
        {
            slice(self.raw_row(col, width).as_ptr() as *const Rgba<u8>, width)
        }
    }
}

/// mutable version of [PixelBuf]
///
/// [PixelBuf]: self::PixelBuf
pub unsafe trait PixelBufMut: PixelBuf
{
    /// mutable version of [PixelBuf::raw_row]
    ///
    /// [PixelBuf::raw_row]: self::PixelBuf::raw_row
    fn raw_row_mut<'a>(&'a mut self, col: usize, width: usize) -> &'a mut [u8];

    /// mutable version of [PixelBuf::row]
    ///
    /// [PixelBuf::row]: self::PixelBuf::row
    #[inline]
    fn row_mut<'a>(&'a mut self, col: usize, width: usize) -> &'a mut [Rgba<u8>]
    {
        use std::slice::from_raw_parts_mut as slice;
        unsafe
        {
            slice(self.raw_row_mut(col, width).as_ptr() as *mut Rgba<u8>, width)
        }
    }
}

/// a promise trait over [PixelBuf] that ensures the
/// implenting type has a reference to the entire buffer
/// of pixels in contiguous memory. that is, row 1 is
/// right before row 2 which is right before row 3, etc.
///
/// [PixelBuf]: self::PixelBuf
pub unsafe trait FlatPixelBuf: PixelBuf
{
    /// get the pixels in `u8` representation, aligned in
    /// memory row-by-row
    fn raw_pixels<'a>(&'a self) -> &'a [u8];

    /// get the pixels aligned in memory row-by-row
    #[inline]
    fn pixels<'a>(&'a self) -> &'a [Rgba<u8>]
    {
        use std::slice::from_raw_parts as slice;

        let buf = self.raw_pixels();
        unsafe
        {
            slice(buf.as_ptr() as *const Rgba<u8>, buf.len() >> 2)
        }
    }
}

/// mutable version of [FlatPixelBuf]
///
/// [FlatPixelBuf]: self::FlatPixelBuf
pub unsafe trait FlatPixelBufMut: FlatPixelBuf
{
    /// get the pixels in `u8` representation, aligned in
    /// memory row-by-row
    fn raw_pixels_mut<'a>(&'a mut self) -> &'a mut [u8];

    /// get the pixels aligned in memory row-by-row
    #[inline]
    fn pixels_mut<'a>(&'a mut self) -> &'a mut [Rgba<u8>]
    {
        use std::slice::from_raw_parts_mut as slice;

        let buf = self.raw_pixels_mut();
        unsafe
        {
            slice(buf.as_ptr() as *mut Rgba<u8>, buf.len() >> 2)
        }
    }
}

unsafe impl<T: AsRef<[u8]>> PixelBuf for T
{
    #[inline]
    fn raw_row<'a>(&'a self, col: usize, width: usize) -> &'a [u8]
    {
        let i = col * width;

        &self.as_ref()[i..i + width]
    }
}

unsafe impl<T: AsRef<[u8]> + AsMut<[u8]>> PixelBufMut for T
{
    #[inline]
    fn raw_row_mut<'a>(&'a mut self, col: usize, width: usize) -> &'a mut [u8]
    {
        let i = col * width;

        &mut self.as_mut()[i..i + width]
    }
}

unsafe impl<T: AsRef<[u8]>> FlatPixelBuf for T
{
    #[inline]
    fn raw_pixels<'a>(&'a self) -> &'a [u8]
    {
        self.as_ref()
    }
}

unsafe impl<T: AsRef<[u8]> + AsMut<[u8]>> FlatPixelBufMut for T
{
    #[inline]
    fn raw_pixels_mut<'a>(&'a mut self) -> &'a mut [u8]
    {
        self.as_mut()
    }
}