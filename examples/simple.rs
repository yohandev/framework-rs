use framework::prelude::*;

fn main()
{
    framework::run::<Foo>();
}

struct Foo;

impl Sketch for Foo
{
    fn setup(app: &mut App) -> Self
    {
        app.create_canvas("Foo", (400, 300));

        Foo
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // clear background to blue
        c.background(Rgba::blue());

        // draw a red line
        c.stroke(c![0xff, 0x00, 0x00]);
        c.line(v![20, 10], v![200, 100]);

        // draw some green lines
        c.stroke(c![0x00, 0xff, 0x00]);
        c.line(v![50, 100], v![0, 10]);
        c.line(v![60, 32], v![90, 67]);

        // draw purple triangle with no stroke
        c.no_stroke();
        c.fill(c![0xff, 0x00, 0xff]);
        c.triangle(v![120, 200], v![20, 30], v![350, 300]);

        // same purple triangle, black stroke
        c.stroke(c![0x00, 0x00, 0x00]);
        c.triangle(v![370, 10], v![370, 230], v![250, 45]);

        // use CSS colour codes.
        // complete list at https://colours.neilorangepeel.com
        c.stroke(c!("teal"));
    }
}