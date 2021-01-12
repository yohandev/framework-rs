use winit::window::{ WindowBuilder, Window as WinitWindow };
use winit::dpi::{ LogicalSize, PhysicalSize };
use winit::event_loop::EventLoopWindowTarget;

use pixels::{ Pixels, SurfaceTexture };

use crate::draw::{ Canvas, Bitmap };
use crate::math::Extent2;

/// represents a window and a pixel buffer
/// attatched to its swapchain
pub struct Window
{
    /// pixels buffer
    pub pixels: Pixels<WinitWindow>,
    /// winit window
    winit: WinitWindow,
    /// pixel buffer size, in pixels
    size: Extent2<usize>,
}

impl Window
{
    /// create a new window
    pub fn new(events: &EventLoopWindowTarget<()>, title: impl Into<String>, size: Extent2<usize>) -> Self
    {
        let logical_size = LogicalSize::new(size.w as f64, size.h as f64);

        let winit = WindowBuilder::new()
            .with_min_inner_size(logical_size)
            .with_inner_size(logical_size)
            .with_title(title)
            .build(events)
            .unwrap();

        let mut pixels =
        {
            let PhysicalSize { width, height } = winit.inner_size();

            let surf_tex = SurfaceTexture::new(width, height, &winit);
            
            Pixels::new(size.w as u32, size.h as u32, surf_tex).unwrap()
        };

        Self
        {
            pixels, winit, size
        }
    }

    /// get the next canvas to draw to
    pub fn get_frame(&mut self) -> Canvas
    {
        let buf = self.pixels.get_frame();
        
        Canvas
        {
            inner: Bitmap::new(buf, self.size),
            window: &self.winit,
        }
    }
}