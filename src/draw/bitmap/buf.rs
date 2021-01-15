/// type that can be used as a bitmap pixel buffer
///
/// while the entire buffer doesn't have to be in
/// contiguous memory, the only requirement is that
/// each row of pixels *is* in contiguous memory.
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
    /// ...`get_col(0)` would return `[A, B, C]`, while
    /// `get_col(3)` would panic.
    ///
    /// safety:
    /// 1. should panic if `row` is out of bounds
    /// 2. the slice returned should always be of constant
    ///    length no matter what value `row` is, assuming
    ///    condition (1) isn't violated.
    /// 3. the slice returned must be safely transmutable
    ///    to `Rgba<u8>`
    fn get_col_raw<'a>(&self) -> &'a [u8];
}

/// a promise trait over [PixelBuf] that ensures the
/// implenting type has a reference to the entire buffer
/// of pixels in contiguous memory. that is, row 1 is
/// right before row 2 which is right before row 3, etc.
pub trait ContiguousPixelBuf: PixelBuf
{

}