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
        background!(c, blue);
        background!(c, Rgba::blue());
        background!(c, [0x00, 0x00, 0xff]);
        background!(c, Rgba::<f32>::blue().map(|n| (n * 255.0) as u8));
    }
}