use crate::util::Barycentric;
use crate::math::*;

/// iterator to trace/rasterize triangles using the bounding box
/// and baryncentric coordinates approach
pub struct Triangle
{
    // (current x, current y)
    cur: Vec2<i32>,

    /// points composing the triangle.
    pts: [Vec2<i32>; 3],

    /// minimum(upper-left corner) of the bounding box
    min: Vec2<i32>,
    /// maximum(lower-right corner) of the bounding box
    max: Vec2<i32>,
}

impl Triangle
{
    /// create a new iterator that yields points inside `pts`
    /// the tuple returned is in (cartesian, barycentric) coordinates
    ///
    /// Differs from [Triangle::new] in that it skips points out of the
    /// canvas's bounds. `size` is higher-bound exclusive.
    ///
    /// [Triangle::new]: crate::util::Triangle::new
    #[inline]
    pub fn new_bounded(pts: [Vec2<i32>; 3], size: Extent2<i32>) -> impl Iterator<Item = (Vec2<i32>, Vec3<f32>)>
    {
        // bounds
        let size: Vec2<i32> = size
            .map(|n| n - 1)
            .into();

        // bounding box
        let mut min: Vec2<i32> = size.clone();
        let mut max: Vec2<i32> = Vec2::zero();

        // compute bounding box
        for vert in &pts
        {
            min = min.map2(*vert, |m, v| m.min(v).max(0));
            max = max.map3(*vert, size, |m, v, b| m.max(v).min(b));
        }

        // starting point
        let cur = min.clone();

        Self { cur, pts, min, max }
    }

    /// create a new iterator that yields points inside `pts`
    #[inline]
    pub fn new(pts: [Vec2<i32>; 3]) -> impl Iterator<Item = (Vec2<i32>, Vec3<f32>)>
    {
        Self::new_bounded(pts, Extent2::broadcast(i32::MAX))
    }
}

impl Iterator for Triangle
{
    type Item = (Vec2<i32>, Vec3<f32>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        // inclusive upper bound
        while self.cur.y <= self.max.y
        {
            // cartesian and barycentric coords
            let p = self.cur;
            let b: Vec3<f32> = p.into_barycentric(self.pts);
            
            // step, either for next Iterator::next call, or for
            // next while loop attempt
            self.cur.x += 1;
            // inclusive upper bound
            if self.cur.x > self.max.x
            {
                self.cur.x = self.min.x;
                self.cur.y += 1;
            }

            // inside the triangle
            if b.x >= 0.0 && b.y >= 0.0 && b.z >= 0.0
            {
                return Some((p, b));
            }
        }
        // done
        None
    }
}

/*use num_traits::{ Signed, PrimInt, Float, AsPrimitive, NumAssignOps };

use crate::util::Barycentric;
use crate::math::*;

/// iterator to trace/rasterize triangles using the bounding box
/// and baryncentric coordinates approach
pub struct Triangle<T, V = f32>
{
    // (current x, current y)
    cur: Vec2<T>,

    /// points composing the triangle.
    pts: [Vec2<T>; 3],

    /// minimum(upper-left corner) of the bounding box
    min: Vec2<T>,
    /// maximum(lower-right corner) of the bounding box
    max: Vec2<T>,

    /// marks what type of float this iterator uses in its
    /// barycentric coordinates
    f: std::marker::PhantomData<V>
}

impl<V: Float + 'static, T: Signed + PrimInt + AsPrimitive<V> + NumAssignOps> Triangle<T, V>
{
    /// create a new iterator that yields points inside `pts`
    /// the tuple returned is in (cartesian, barycentric) coordinates
    ///
    /// Differs from [Triangle::new] in that it skips points out of the
    /// canvas's bounds. `size` is higher-bound exclusive.
    ///
    /// [Triangle::new]: crate::util::Triangle::new
    #[inline]
    pub fn new_bounded(pts: [Vec2<T>; 3], size: Extent2<T>) -> impl Iterator<Item = (Vec2<T>, Vec3<V>)>
    {
        // bounds
        let size: Vec2<T> = size
            .map(|n| n - T::one())
            .into();

        // bounding box
        let mut min: Vec2<T> = size.clone();
        let mut max: Vec2<T> = Vec2::zero();

        // compute bounding box
        for vert in &pts
        {
            min = min.map2(*vert, |m, v| m.min(v).max(T::zero()));
            max = max.map3(*vert, size, |m, v, b| m.max(v).min(b));
        }

        // starting point
        let cur = min.clone();
        // phantom
        let f = Default::default();

        Self { cur, pts, min, max, f }
    }

    /// create a new iterator that yields points inside `pts`
    #[inline]
    pub fn new(pts: [Vec2<T>; 3]) -> impl Iterator<Item = (Vec2<T>, Vec3<V>)>
    {
        Self::new_bounded(pts, Extent2::broadcast(T::max_value()))
    }
}

impl<V: Float + 'static, T: Signed + PrimInt + AsPrimitive<V> + NumAssignOps> Iterator for Triangle<T, V>
{
    type Item = (Vec2<T>, Vec3<V>);

    #[inline]
    fn next(&mut self) -> Option<Self::Item>
    {
        // inclusive upper bound
        while self.cur.y <= self.max.y
        {
            // cartesian and barycentric coords
            let p = self.cur;
            let b: Vec3<V> = p.into_barycentric(self.pts);
            
            // step, either for next Iterator::next call, or for
            // next while loop attempt
            self.cur.x += T::one();
            // inclusive upper bound
            if self.cur.x > self.max.x
            {
                self.cur.x = self.min.x;
                self.cur.y += T::one();
            }

            // inside the triangle
            if b.x >= V::zero() && b.y >= V::zero() && b.z >= V::zero()
            {
                return Some((p, b));
            }
        }
        // done
        None
    }
}*/