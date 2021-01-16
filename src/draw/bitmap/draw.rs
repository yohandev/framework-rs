use crate::draw::{ Bitmap, PixelBuf, PixelBufMut };
use crate::math::{ Vec2, Rgba };

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
            incremental_fill(buf, col);
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
            incremental_fill(row0, col);

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

    /// draws a line from `a` to `b`, clipping any pixels out of
    /// bounds
    pub fn line(&mut self, a: Vec2<i32>, b: Vec2<i32>)
    {
        use crate::util::Bresenham;

        // stroke
        if let Some(stroke) = self.stroke
        {
            for pos in Bresenham::new_bounded(a, b, self.size().as_())
            {
                self[pos] = stroke;
            }
        }
    }

    /// draws a triangle with vertices `a`, `b`, and `c`, clipping
    /// any pixels out of bounds
    pub fn triangle(&mut self, a: Vec2<i32>, b: Vec2<i32>, c: Vec2<i32>)
    {
        use crate::util::Triangle;

        // fill
        if let Some(fill) = self.fill
        {
            for (pos, _) in Triangle::new_bounded([a, b, c], self.size().as_())
            {
                self[pos] = fill;
            }
        }

        // stroke
        self.line(a, b);
        self.line(b, c);
        self.line(c, a);
    }

    /// draws a rectangle with top-left corner at `pos` and of
    /// (width, height) `siz`. clips any pixels out of bounds.
    pub fn rect(&mut self, pos: Vec2<i32>, siz: Vec2<i32>)
    {
        // size of this bitmap
        let bounds: Vec2<i32> = self.size.as_().into();

        // crop with top left corner(0, 0)
        let siz = siz.map2(pos, |s, p| if p < 0 { s + p } else { s });
        let pos = pos.map(|n| n.max(0));
        // crop with bottom right corner(width - 1, height - 1)
        let siz = siz.map3(pos, bounds, |siz, pos, bound|
        {
            let d = bound - (pos + siz);
            if d < 0 { siz + d } else { siz }
        });
        
        // empty rectangle
        if siz.x <= 0 || siz.y <= 0
        {
            return;
        }

        // fill
        if let Some(fill) = self.fill
        {
            // conversions are now safe
            let pos: Vec2<usize> = pos.as_();
            let siz: Vec2<usize> = siz.as_();

            // get the first row
            let row0 = 
            {
                // SAFETY: borrow checker isn't smart enough to know
                // we're only borrowing the first row once
                let this = &*self as *const Self as *mut Self;

                unsafe { &mut *this }.buf.row_mut(pos.y, self.width())
            };
            // get only the columns we care about in the first row
            let row0 = &mut row0[pos.x..pos.x + siz.x];

            // fill the first row
            incremental_fill(row0, fill);

            // fill every other row
            for y in pos.y + 1..pos.y + siz.y
            {
                // get the row
                let row = self.buf.row_mut(y, self.width());

                // get only the columns we care about
                let row = &mut row[pos.x..pos.x + siz.x];

                // copy the first row into this one
                row.copy_from_slice(row0);
            }
        }
        // stroke
        if self.stroke.is_some()
        {
            let top_l = pos;
            let btm_l = pos + Vec2::new(0, siz.y);
            let top_r = pos + Vec2::new(siz.x, 0);
            let btm_r = pos + siz;

            self.line(top_l, top_r);
            self.line(btm_l, btm_r);
            self.line(top_l, btm_l);
            self.line(top_r, btm_r);
        }
    }

    /// paste another bitmap on top of this one, clipping any invisible
    /// pixels and (optionally) translating it
    ///
    /// the source bitmap isn't affected
    pub fn image<J>(&mut self, src: &Bitmap<J, impl PixelBuf>, pos: Vec2<i32>)
    {
        // givens
        let dst_size: Vec2<i32> = self.size().as_::<i32>().into();
        let src_size: Vec2<i32> = src.size().as_::<i32>().into();

        // as you iterate src's pixels; [0, src_width] and [0, src_height]
        let src_min = pos.map2(src_size, |p, s| (if p < 0 { -p } else { 0 }).min(s));
        let src_max = pos.map3(src_size, dst_size, |p, ss, ds| if p + ss > ds { ds - p } else { ss });
  
        // as you copy to dst's pixels; [0, dst_width] and [0, dst_height]
        let dst_min_x = if pos.x < 0 { 0 } else { pos.x } as usize;
        let dst_max_x = dst_min_x + (src_max.x - src_min.x) as usize;

        // nothing to copy
        if dst_max_x < dst_min_x
        {
            return;
        }

        // now safe to convert
        let src_min: Vec2<usize> = src_min.as_();
        let src_max: Vec2<usize> = src_max.as_();

        // iterate vertically
        for y in src_min.y..src_max.y
        {
            // get the source image's row 
            let src_buf = src.buf.row(y, src.width());
            // take only the columns we care about
            let src_buf = &src_buf[src_min.x..src_max.x];

            // get the destination image's row
            let dst_buf = self.buf.row_mut(y, self.width());
            // take only the columns we care about
            let dst_buf = &mut dst_buf[dst_min_x..dst_max_x];

            // copy entire row at once
            dst_buf.copy_from_slice(src_buf);
        }
    }
}

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