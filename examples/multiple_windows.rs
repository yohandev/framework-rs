use framework::prelude::*;

fn main()
{
    framework::run::<Bar>();
}

struct Bar(CanvasId, CanvasId, CanvasId);

impl Sketch for Bar
{
    fn setup(app: &mut App) -> Self
    {
        let c1 = app.create_canvas("Bar 1", (400, 300));
        let c2 = app.create_canvas("Bar 2", (200, 200));
        let c3 = app.create_canvas("Bar 3", (600, 400));

        Bar(c1, c2, c3)
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        if c.id() == &self.0
        {
            c.background(Rgba::blue());
        }
        else if c.id() == &self.1
        {
            c.background(Rgba::yellow());
        }
        else if c.id() == &self.2
        {
            c.background(Rgba::green());
        }
    }
}