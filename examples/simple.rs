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
        background!(c, Rgba::blue());

        // draw a red line
        stroke!(c, (0xff, 0x00, 0x00));
        lines!(c, [20, 10], [200, 100]);

        // draw some green lines
        stroke!(c, (0x00, 0x00, 0xff));
        lines!(c, ([50, 100], [0, 10]), ([60, 32], [90, 67]));

        // draw purple triangle with no stroke
        stroke!(c, None);
        fill!(c, (0xff, 0x00, 0xff));
        
    }
}