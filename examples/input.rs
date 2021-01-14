use framework::prelude::*;

fn main()
{
    framework::run::<Baz>();
}

struct Baz
{
    mouse_pos: Vec2<i32>,
}

impl Sketch for Baz
{
    fn setup(app: &mut App) -> Self
    {
        app.create_canvas("baz", (600, 400));

        Self { mouse_pos: v![0, 0] }
    }

    fn update(&mut self, app: &mut App)
    {
        app.time().print_fps();

        self.mouse_pos = app.mouse().position();
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        c.background(c!("darkslategray"));

        c.fill(c!("peru"));
        c.no_stroke();

        c.triangle(v![0, 50], v![0, 349], self.mouse_pos);
    }
}