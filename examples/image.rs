use framework::prelude::*;

fn main()
{
    framework::run::<Baz>();
}

struct Baz(Image);

impl Sketch for Baz
{
    fn setup(app: &mut App) -> Self
    {
        let img = app
            .load_image("examples/res/trees.jpg")
            .unwrap();

        app.create_canvas("happy little trees", img.size());

        Self(img)
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        c.image(&self.0, v![0, 0]);
    }
}