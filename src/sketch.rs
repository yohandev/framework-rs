use crate::draw::Canvas;
use crate::App;

/// represents a sketch that can be run by the framework
pub trait Sketch: 'static
{
    /// setup this sketch, creating canvas(es)
    /// and loading additional resources
    fn setup(app: &mut App) -> Self;

    /// draw on the given `Canvas`. This method
    /// is called for every existing canvas on
    /// every frame
    fn draw(&mut self, c: &mut Canvas)
    {
        c.background(Default::default());
    }

    /// update the state of this `Sketch` and its
    /// `App`. This method is called once every
    /// frame
    fn update(&mut self, app: &mut App)
    {
        app.time().print_fps();
    }
}