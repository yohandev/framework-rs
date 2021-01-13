/// clears the entire [Canvas] with the given
/// colour.
/// ```
/// // all four of these work!
/// background!(c, blue);
/// background!(c, Rgba::blue());
/// background!(c, [0x00, 0x00, 0xff]);
/// background!(c, Rgba::<f32>::blue().map(|n| (n * 255.0) as u8));
/// ```
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