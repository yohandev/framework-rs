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
        let blue = Rgba::blue();

        // all four of these work!
        stroke!(c, blue);
        fill!(c, Rgba::blue());
        fill!(c, [0x00, 0x00, 0xff]);
        fill!(c, Rgba::<f32>::blue().map(|n| (n * 255.0) as u8));
    }
}