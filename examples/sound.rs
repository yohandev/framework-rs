#[cfg(feature = "audio")]
use framework::prelude::*;

#[cfg(feature = "audio")]
fn main()
{
    framework::run::<Foo>();
}

#[cfg(not(feature = "audio"))]
fn main()
{
    panic!("audio feature must be enabled!");
}

#[cfg(feature = "audio")]
struct Foo(Track<f32>);

#[cfg(feature = "audio")]
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