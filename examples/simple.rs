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
        c.stroke([0xff, 0x00, 0x00]);
        c.line([20, 10], [200, 100]);

        // draw some green lines
        c.stroke([0x00, 0xff, 0x00]);
        c.line([50, 100], [0, 10]);
        c.line([60, 32], [90, 67]);

        // draw purple triangle with no stroke
        c.no_stroke();
        c.fill([0xff, 0x00, 0xff]);
        c.triangle([120, 200], [20, 30], [350, 300]);

        // same purple triangle, black stroke
        c.stroke([0x00, 0x00, 0x00]);
        c.triangle([370, 10], [370, 230], [250, 45]);
    }
}