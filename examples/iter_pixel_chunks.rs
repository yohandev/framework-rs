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
        // erase everything only the first time
        if self.1 == 0 
        {
            c.background(c!("dodgerblue"));
        }

        // // go through each chunk
        // for (i, chunk) in self.0
        //     .iter_pixel_chunks(v![40, 40])
        //     .enumerate()
        // {
        //     // only draw the current chunk
        //     if self.1 == i
        //     {
        //         // `Chunk` isn't `Bitmap` compatible yet...
        //         for x in 0..40
        //         {
        //             for y in 0..40
        //             {
        //                 // do this the old fashion way... pixel by pixel
        //                 c[chunk.pos() + v![x, y]] = chunk[v![x, y]];
        //             }
        //         }
        //         break;
        //     }   
        // }
        self.1 += 1;
    }
}