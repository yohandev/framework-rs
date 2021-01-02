use num_traits::Num;

/// represents a vector with 2 elements
pub trait Vec2<T>
{
    /// get the first(of two) component in this vector
    fn x(&self) -> T where T: Copy;
    /// get the second(of two) component in this vector
    fn y(&self) -> T where T: Copy;

    /// set the first(of two) component in this vector
    fn set_x(&mut self, x: T);
    /// set the second(of two) component in this vector
    fn set_y(&mut self, y: T);
}

impl<T: Num> Vec2<T> for (T, T)
{
    #[inline]
    fn x(&self) -> T where T: Copy { self.0 }

    #[inline]
    fn y(&self) -> T where T: Copy { self.1 }

    #[inline]
    fn set_x(&mut self, x: T) { self.0 = x; }

    #[inline]
    fn set_y(&mut self, y: T) { self.1 = y; }
}