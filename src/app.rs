use crate::{ Frame, Time, math::Extent2 };

/// represents an application that can be run by the framework
pub trait App: 'static
{
    /// the name of this app, which will be displayed as the window
    /// title
    const TITLE: &'static str = "Playground";

    /// the default width and height(in pixels) for the window and
    /// framebuffer
    const SIZE: Extent2<usize> = Extent2::new(600, 400);

    /// render to the window
    fn render(&mut self, frame: &mut Frame);

    /// update the state of the app
    fn update(&mut self, time: &Time);
}