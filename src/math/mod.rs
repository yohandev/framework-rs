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

/// creates a `framework::math::Rgba<u8>` with alpha
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
/// let a = Rgba::<u8>::new(0xff, 0x12, 0x40);
/// let b = Rgba::<u8>::new(0xff, 0x12, 0x40, 0xff);
/// // translucent
/// let c = Rgba::<u8>::new(0xff, 0x12, 0x40, 0x80);
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
        framework::math::Rgba::<u8>::new($r, $g, $b, $a)
    };
    [$r:expr, $g:expr, $b:expr] =>
    {
        framework::math::Rgba::<u8>::new($r, $g, $b, 0xff)
    };
    ("teal") => { c![0,128,128] };
    ("seagreen") => { c![46,139,87] };
    ("honeydew") => { c![240,255,240] };
    ("lemonchiffon") => { c![255,250,205] };
    ("peru") => { c![205,133,63] };
    ("darkseagreen") => { c![143,188,143] };
    ("oldlace") => { c![253,245,230] };
    ("wheat") => { c![245,222,179] };
    ("lavender") => { c![230,230,250] };
    ("beige") => { c![245,245,220] };
    ("lightseagreen") => { c![32,178,170] };
    ("lightslategray") => { c![119,136,153] };
    ("aquamarine") => { c![127,255,212] };
    ("lightsalmon") => { c![255,160,122] };
    ("aliceblue") => { c![240,248,255] };
    ("dodgerblue") => { c![30,144,255] };
    ("orange") => { c![255,165,0] };
    ("crimson") => { c![220,20,60] };
    ("indianred") => { c![205,92,92] };
    ("fuchsia") => { c![255,0,255] };
    ("maroon") => { c![128,0,0] };
    ("palevioletred") => { c![219,112,147] };
    ("peachpuff") => { c![255,218,185] };
    ("olivedrab") => { c![107,142,35] };
    ("darkslategray") => { c![47,79,79] };
    ("blue") => { c![0,0,255] };
    ("paleturquoise") => { c![175,238,238] };
    ("lightslategrey") => { c![211,211,211] };
    ("cornflowerblue") => { c![100,149,237] };
    ("royalblue") => { c![65,105,225] };
    ("yellow") => { c![255,255,0] };
    ("olive") => { c![128,128,0] };
    ("steelblue") => { c![70,130,180] };
    ("palegreen") => { c![152,251,152] };
    ("darkviolet") => { c![148,0,211] };
    ("mediumorchid") => { c![186,85,211] };
    ("darkblue") => { c![0,0,139] };
    ("tan") => { c![210,180,140] };
    ("mediumpurple") => { c![147,112,219] };
    ("brown") => { c![165,42,42] };
    ("gray") => { c![128,128,128] };
    ("midnightblue") => { c![25,25,112] };
    ("palegoldenrod") => { c![238,232,170] };
    ("mistyrose") => { c![255,228,225] };
    ("mediumvioletred") => { c![199,21,133] };
    ("gold") => { c![255,215,0] };
    ("turquoise") => { c![64,224,208] };
    ("dimgrey") => { c![105,105,105] };
    ("lawngreen") => { c![124,252,0] };
    ("darkolivegreen") => { c![85,107,47] };
    ("aqua") => { c![0,255,255] };
    ("gainsboro") => { c![220,220,220] };
    ("darkgreen") => { c![0,100,0] };
    ("ivory") => { c![255,255,240] };
    ("lavenderblush") => { c![255,240,245] };
    ("rosybrown") => { c![188,143,143] };
    ("lightsteelblue") => { c![176,196,222] };
    ("azure") => { c![240,255,255] };
    ("indigo") => { c![75,0,130] };
    ("cyan") => { c![0,255,255] };
    ("lightgrey") => { c![211,211,211] };
    ("darkkhaki") => { c![189,183,107] };
    ("blanchedalmond") => { c![255,235,205] };
    ("limegreen") => { c![50,205,50] };
    ("darkorchid") => { c![153,50,204] };
    ("darksalmon") => { c![233,150,122] };
    ("salmon") => { c![250,128,114] };
    ("floralwhite") => { c![255,250,240] };
    ("lightyellow") => { c![255,255,224] };
    ("moccasin") => { c![255,228,181] };
    ("plum") => { c![221,160,221] };
    ("white") => { c![255,255,255] };
    ("blueviolet") => { c![138,43,226] };
    ("darkslategrey") => { c![47,79,79] };
    ("springgreen") => { c![0,255,127] };
    ("forestgreen") => { c![34,139,34] };
    ("khaki") => { c![240,230,140] };
    ("thistle") => { c![216,191,216 ] };
    ("slategray") => { c![112,128,144] };
    ("chartreuse") => { c![127,255,0] };
    ("lightgray") => { c![211,211,211] };
    ("sienna") => { c![160,82,45] };
    ("skyblue") => { c![135,206,235] };
    ("rebeccapurple") => { c![102,51,153] };
    ("goldenrod") => { c![218,165,32] };
    ("lime") => { c![0,255,0] };
    ("deeppink") => { c![255,20,147] };
    ("cadetblue") => { c![95,158,160] };
    ("greenyellow") => { c![173,255,47] };
    ("navajowhite") => { c![255,222,173] };
    ("lightpink") => { c![255,182,193] };
    ("darkturquoise") => { c![0,206,209] };
    ("darkmagenta") => { c![139,0,139] };
    ("purple") => { c![128,0,128] };
    ("lightskyblue") => { c![135,206,250] };
    ("black") => { c![0,0,0] };
    ("sandybrown") => { c![244,164,96] };
    ("lightblue") => { c![173,216,230] };
    ("bisque") => { c![255,228,196] };
    ("orchid") => { c![218,112,214] };
    ("mediumaquamarine") => { c![102,205,170] };
    ("burlywood") => { c![222,184,135] };
    ("orangered") => { c![255,69,0] };
    ("violet") => { c![238,130,238] };
    ("mediumblue") => { c![0,0,205] };
    ("mediumseagreen") => { c![60,179,113] };
    ("darkgray") => { c![169,169,169] };
    ("powderblue") => { c![176,224,230] };
    ("mediumturquoise") => { c![72,209,204] };
    ("whitesmoke") => { c![245,245,245] };
    ("red") => { c![255,0,0] };
    ("papayawhip") => { c![255,239,213] };
    ("mediumslateblue") => { c![123,104,238] };
    ("darkorange") => { c![255,140,0] };
    ("silver") => { c![192,192,192] };
    ("green") => { c![0,128,0] };
    ("chocolate") => { c![210,105,30] };
    ("darkred") => { c![139,0,0] };
    ("pink") => { c![255,192,203] };
    ("darkslateblue") => { c![72,61,139] };
    ("seashell") => { c![255,245,238] };
    ("antiquewhite") => { c![250,235,215] };
    ("hotpink") => { c![255,105,180] };
    ("slategrey") => { c![112,128,144] };
    ("lightgreen") => { c![144,238,144] };
    ("snow") => { c![255,250,250] };
    ("yellowgreen") => { c![154,205,50] };
    ("saddlebrown") => { c![139,69,19] };
    ("lightcyan") => { c![224,255,255] };
    ("deepskyblue") => { c![0,191,255] };
    ("cornsilk") => { c![255,248,220] };
    ("magenta") => { c![255,0,255] };
    ("mintcream") => { c![245,255,250] };
    ("ghostwhite") => { c![248,248,255] };
    ("firebrick") => { c![178,34,34] };
    ("lightgoldenrodyellow") => { c![250,250,210] };
    ("lightcoral") => { c![240,128,128] };
    ("darkcyan") => { c![0,139,139] };
    ("tomato") => { c![255,99,71] };
    ("navy") => { c![0,0,128] };
    ("mediumspringgreen") => { c![0,250,154] };
    ("linen") => { c![250,240,230] };
    ("slateblue") => { c![106,90,205] };
    ("darkgoldenrod") => { c![184,134,11] };
    ("dimgray") => { c![105,105,105] };
    ("coral") => { c![255,127,80] };
    ("darkgrey") => { c![169,169,169] };
    ("grey") => { c![80, 80, 80] };
}