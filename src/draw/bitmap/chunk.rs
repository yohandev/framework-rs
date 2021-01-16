// use std::ops::Index;

// use crate::math::{ Vec2, Rgba, Extent2 };
// use crate::draw::{ Bitmap };

// pub struct Chunk<'a>
// {
//     /// outer-most array is columns, where
//     /// each element is a reference to a
//     /// row
//     /// ```text
//     /// buf: Box -> [&A, &B, &C]
//     ///     &A -> [(0, 0), (1, 0), (2, 0)]
//     ///     &B -> [(0, 1), (1, 1), (2, 1)]
//     ///     &C -> [(0, 2), (1, 2), (2, 2)]
//     /// where &A, &B, and &C are assumed to be
//     /// in non-contiguous memory
//     /// ```
//     buf: Box<[&'a [Rgba<u8>]]>,
//     /// position, in pixel space, of the top-left
//     /// corner of this chunk
//     pos: Vec2<i32>,
// }

// /// `Bitmap::iter_pixel_chunks` implementation
// pub(crate) fn iter_pixel_chunks<I, B>(bitmap: &Bitmap<I, B>, size: Extent2<usize>) -> impl Iterator<Item = Chunk<'_>>
// {
//     // iterate in row-by-row zig-zag pattern
//     (0..bitmap.height() / size.h)
//         // do the cartesian product
//         .flat_map(move |y| (0..bitmap.width() / size.w).map(move |x| (x, y)))
//         // once we have zig-zag indices, begin dividing chunks:
//         .map(move |(x, y)|
//         {
//             // (x, y) is chunk index; remap to top-left corner in pixel
//             // space
//             let pos = Vec2::new(x * size.w, y * size.h);

//             // create sparse 2D buffer(see `Chunks::buf` doc)
//             let buf = (pos.y..pos.y + size.h)
//                 // go through each row in chunk
//                 .map(|y|
//                 {
//                     // starting index
//                     let i = y * bitmap.width() + pos.x;
//                     // take borrow
//                     &bitmap.pixels()[i..i + size.w]
//                 })
//                 .collect::<Box<_>>();
            
//             // convert pos
//             let pos = pos.as_();
            
//             // return chunks
//             Chunk { buf, pos }
//         })
// }

// impl<'a> Chunk<'a>
// {
//     /// get the position, in pixel space, of the top-left
//     /// corner of this `Chunk`
//     #[inline]
//     pub fn pos(&self) -> Vec2<i32>
//     {
//         self.pos
//     }
// }

// impl<'a> Index<Vec2<i32>> for Chunk<'a>
// {
//     type Output = Rgba<u8>;
    
//     #[inline]
//     fn index(&self, pos: Vec2<i32>) -> &Rgba<u8>
//     {
//         &self.buf[pos.y as usize][pos.x as usize]
//     }
// }