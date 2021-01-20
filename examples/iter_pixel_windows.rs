use framework::prelude::*;

fn main()
{
    framework::run::<Foo>();
}

struct Foo(Image, usize);

impl Sketch for Foo
{
    fn setup(app: &mut App) -> Self
    {
        let img = app
            .load_image("examples/res/trees.jpg")
            .unwrap();

        app.create_canvas("chunks!", img.size());

        Self(img, 0)
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // erase everything everytime
        c.background(c!("dodgerblue"));

        // go through each chunk
        for (i, chunk) in self.0
            .iter_pixel_windows(v![40, 40].into())
            .enumerate()
        {
            // only draw the current chunk
            if self.1 == i
            {
                c.image(&chunk, *chunk.id());
                break;
            }   
        }
        self.1 += 1;
    }
}