pub mod input;
pub mod sound;
pub mod draw;
pub mod util;
pub mod math;
pub mod core;

pub mod prelude
{
    pub use crate::math::vec::
    {
        Vec2,
        Vec3,
        Vec4,
        Rgba,
        Extent2,
    };

    pub use crate::sound::
    {
        Track,
    };
    pub use crate::core::
    {
        run,

        App,
        Sketch,
    };
    pub use crate::draw::
    {
        ParallelIterator,
        Image,
        Canvas,
    };
}