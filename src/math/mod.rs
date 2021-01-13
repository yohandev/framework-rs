pub use vek::*;

/// creates a `framework::math::Vec*` based on the
/// number or arguments. Usage like this:
/// ```
/// let a = v![30, 10];
/// ```
/// ...would be shorthand for something like this:
/// ```
/// let a = Vec2::new(30, 10);
/// ```
/// this macro is to get clean vector-creation
/// syntax without abusing the `Into` trait for
/// every usage of vectors, like so:
/// ```
/// fn foo(a: impl Into<Vec2<i32>>)
/// {
///     // this would, effectively work with arrays
///     // and tuples input, but the funciton has to
///     // convert every single time
///     let a: Vec2<i32> = a.into();
/// }
/// ```
#[macro_export]
macro_rules! v
{
    [$x:expr, $y:expr, $z:expr, $w:expr] =>
    {
        framework::math::Vec4::new($x, $y, $z, $w)
    };
    [$x:expr, $y:expr, $z:expr] =>
    {
        framework::math::Vec3::new($x, $y, $z)
    };
    [$x:expr, $y:expr] =>
    {
        framework::math::Vec2::new($x, $y)
    };
}

/// creates a `framework::math::Rgba` with alpha
/// based on the number or arguments. Usage like this:
/// ```
/// // fully opaque
/// let a = c![0xff, 0x12, 0x40];
/// let b = c![0xff, 0x12, 0x40, 0xff];
/// // translucent
/// let c = c![0xff, 0x12, 0x40, 0x80];
/// ```
/// ...would be shorthand for something like this:
/// ```
/// // fully opaque
/// let a = Rgba::new_opaque(0xff, 0x12, 0x40);
/// let b = Rgba::new(0xff, 0x12, 0x40, 0xff);
/// // translucent
/// let c = Rgba::new(0xff, 0x12, 0x40, 0x80);
/// ```
/// this macro is to get clean colour-creation
/// syntax without abusing the `Into` trait for
/// every usage of colours, like so:
/// ```
/// fn foo(a: impl Into<Rgba<u8>>)
/// {
///     // this would, effectively work with arrays
///     // and tuples input, but the funciton has to
///     // convert every single time
///     let a: Rgba<u8> = a.into();
/// }
/// ```
/// the `c!` macro also has syntax over its `v!`
/// counterpart, namely for constant colours:
/// ```
/// let a = c!("black");
/// let b = c![0x00, 0x00, 0x00];
/// 
/// assert_eq!(a, b);
/// ```
/// ...or hex codes:
/// ```
/// let a = c!("#F0FFF0");
/// let b = c![0xf0, 0xff, 0xf0];
///
/// assert_eq!(a, b);
/// ```
#[macro_export]
macro_rules! c
{
    [$r:expr, $g:expr, $b:expr, $a:expr] =>
    {
        framework::math::Rgba::new($r, $g, $b, $a)
    };
    [$r:expr, $g:expr, $b:expr] =>
    {
        framework::math::Rgba::new_opaque($r, $g, $b)
    };
    ("teal") =>
    {
        framework::math::Rgba::<u8>::new_opaque(0, 128, 128)
    };
    
}