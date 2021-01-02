use crate::math::*;

/// iterator to trace/draw lines using Bresenham's line drawing
/// algorithm
pub struct Bresenham
{
    // (current x, current y)
    cur: Vec2<i32>,

    // delta.x
    dx: i32,
    // delta.y.signum()
    dy: i32,

    // error
    e: i32,
    // delta error
    de: i32,

    // whether to flip (x,y) to (y, x)
    steep: bool,

    // last x position, inclusive
    end: i32,
}

impl Bresenham
{
    /// create a new iterator that yields points from a to b, inclusive
    #[inline]
    pub fn new(mut a: Vec2<i32>, mut b: Vec2<i32>) -> impl Iterator<Item = Vec2<i32>>
    {
        // adjust slope
        let steep = if (a.x - b.x).abs() < (a.y - b.y).abs()
        {
            std::mem::swap(&mut a.x, &mut a.y);
            std::mem::swap(&mut b.x, &mut b.y);

            true
        }
        else
        {
            false
        };

        // flip the x so that we always start with the lowest x
        if a.x > b.x
        {
            std::mem::swap(&mut a.x, &mut b.x); 
            std::mem::swap(&mut a.y, &mut b.y); 
        }

        // delta
        let d = b - a;
        
        Self
        {
            cur: a,             // starting x and y
            dx: d.x,            // delta x
            dy: d.y.signum(),   // delta y
            e: 0,               // slope error(0.5)
            de: d.y.abs() * 2,  // slope error increment(0.5)
            steep,              // steep or not
            end: b.x,           // ending x
        }
    }

    /// create a new iterator that yields points from a to b, inclusive
    ///
    /// Differs from [Bresenham::new] in that it skips points
    /// out of the canvas's bounds. `size` is higher-bound exclusive.
    ///
    /// [Bresenham::new]: crate::util::Bresenham::new
    #[inline]
    pub fn new_bounded(a: Vec2<i32>, b: Vec2<i32>, size: Extent2<i32>) -> impl Iterator<Item = Vec2<i32>>
    {
        Self::new(a, b)
            .filter(move |p| p.x >= 0 && p.y >= 0 && p.x < size.w && p.y < size.h)
    }
}

impl Iterator for Bresenham
{
    type Item = Vec2<i32>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        let out;
        if self.cur.x > self.end
        {
            // done
            out = None;
        }
        else
        {
            // next pixel
            out = Some(if self.steep { self.cur.yx() } else { self.cur });

            // increment slope error
            self.e += self.de; 
            if self.e > self.dx
            {
                self.cur.y += self.dy;
                self.e -= self.dx * 2;
            }
            // increment x
            self.cur.x += 1;
        }
        out
    }
}