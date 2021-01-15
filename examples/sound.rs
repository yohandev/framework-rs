use framework::prelude::*;

fn main()
{
    framework::run::<Foo>();
}

struct Foo(Track<f32>);

impl Sketch for Foo
{
    fn setup(app: &mut App) -> Self
    {
        app.create_canvas("piano!", (300, 200));

        let song = app
            .load_sound("examples/res/piano.ogg")
            .unwrap();

        Self(song)
    }

    fn update(&mut self, app: &mut App)
    {
        if app.keys().pressed(btn!(" "))
        {
            self.0.toggle_play();
        }
    }
}