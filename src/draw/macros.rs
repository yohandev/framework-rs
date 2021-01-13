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
    ($canvas:expr, [$x:expr, $y:expr, $z:expr, $w:expr]) =>
    {
        $canvas.background(framework::math::Rgba::new($x, $y, $z, $w))
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

/// sets the colour to be used for any future drawing operations
/// ```
/// // all four of these work!
/// fill!(c, blue);
/// fill!(c, Rgba::blue());
/// fill!(c, [0x00, 0x00, 0xff]);
/// fill!(c, Rgba::<f32>::blue().map(|n| (n * 255.0) as u8));
/// ```
#[macro_export]
macro_rules! fill
{
    ($canvas:expr, [$x:expr, $y:expr, $z:expr, $w:expr]) =>
    {
        $canvas.set_fill(Some(framework::math::Rgba::new($x, $y, $z, $w)))
    };
    ($canvas:expr, [$x:expr, $y:expr, $z:expr]) =>
    {
        $canvas.set_fill(Some(framework::math::Rgba::new($x, $y, $z, 0xff)))
    };
    ($canvas:expr, $col:expr) =>
    {
        $canvas.set_fill($col)
    };
}

/// sets the colour to be used for any future drawing operations
/// ```
/// // all four of these work!
/// stroke!(c, blue);
/// stroke!(c, Rgba::blue());
/// stroke!(c, [0x00, 0x00, 0xff]);
/// stroke!(c, Rgba::<f32>::blue().map(|n| (n * 255.0) as u8));
/// ```
#[macro_export]
macro_rules! stroke
{
    ($canvas:expr, [$x:expr, $y:expr, $z:expr, $w:expr]) =>
    {
        $canvas.set_stroke(Some(framework::math::Rgba::new($x, $y, $z, $w)))
    };
    ($canvas:expr, [$x:expr, $y:expr, $z:expr]) =>
    {
        $canvas.set_stroke(Some(framework::math::Rgba::new($x, $y, $z, 0xff)))
    };
    ($canvas:expr, $col:expr) =>
    {
        $canvas.set_stroke($col)
    };
}

/// draw line(s) to the given canvas. this macro is called
/// `lines` to disambiguate with `std::line`, and, as a result
/// also supports drawing multiple lines at once.
/// ```
/// // draw one line from [x: 20, y: 10] to [x: 200, y: 100]
/// lines!(c, [20, 10], [200, 100]);
/// // draw two lines
/// lines!(c, ([50, 100], [0, 10]), ([60, 32], [90, 67]));
/// ```
#[macro_export]
macro_rules! lines
{
    ($canvas:expr, [$ax:expr, $ay:expr], [$bx:expr, $by:expr]) =>
    {
        $canvas.line(framework::math::Vec2::new($ax, $ay), framework::math::Vec2::new($bx, $by))
    };
    ($canvas:expr, $(([$ax:expr, $ay:expr], [$bx:expr, $by:expr])),*) =>
    {
        $($canvas.line(framework::math::Vec2::new($ax, $ay), framework::math::Vec2::new($bx, $by)));*
    };
}

/// draws a triangle with vertices `a`, `b`, and `c`, clipping
/// any pixels out of bounds.
#[macro_export]
macro_rules! triangle
{
    ($canvas:expr, [$ax:expr, $ay:expr], [$bx:expr, $by:expr], [$cx:expr, $cy:expr]) =>
    {
        $canvas.triangle(
            framework::math::Vec2::new($ax, $ay),
            framework::math::Vec2::new($bx, $by),
            framework::math::Vec2::new($cx, $cy),
        )
    };
}