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
        background!(c, Rgba::blue());

        stroke!(c, Rgba::red());
        lines!(c, [20, 10], [200, 100]);

        stroke!(c, Rgba::yellow());
        lines!(c, ([50, 100], [0, 10]), ([60, 32], [90, 67]));
    }
}