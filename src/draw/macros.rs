// clears the entire [Canvas] with the given
/// colour.
///
/// ```
/// background!(c, [0x30, 0x40, 0x1, 0xff]);
/// background!(c, BLUE);
/// background!(c, Rgba::<f32>::blue().map(|n| (n * 255.0) as u8));
/// ```
///
/// [Canvas]: framework::prelude::Canvas
#[macro_export]
macro_rules! background
{
    ($canvas:expr, ($x:expr, $y:expr, $z:expr, $w:expr)) =>
    {
        $canvas.background(framework::math::Rgba::new($x, $y, $z, $w))
    };
    ($canvas:expr, [$x:expr, $y:expr, $z:expr, $w:expr]) =>
    {
        $canvas.background(framework::math::Rgba::new($x, $y, $z, $w))
    };
    ($canvas:expr, ($x:expr, $y:expr, $z:expr)) =>
    {
        $canvas.background(framework::math::Rgba::new($x, $y, $z, 0xff))
    };
    ($canvas:expr, [$x:expr, $y:expr, $z:expr]) =>
    {
        $canvas.background(framework::math::Rgba::new($x, $y, $z, 0xff))
    };
    ($canvas:expr, $col:expr) =>
    {
        $canvas.background($col)
    };
}