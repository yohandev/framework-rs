use winit::window::{ WindowBuilder, Window as WinitWindow };
use winit::dpi::{ LogicalSize, PhysicalSize };
use winit::event_loop::EventLoopWindowTarget;

use pixels::{ Pixels, SurfaceTexture };

use crate::draw::{ Canvas, CanvasId, Bitmap };
use crate::math::Extent2;

/// represents a window and a pixel buffer
/// attatched to its swapchain
#[derive(Debug)]
pub(crate) struct Window
{
    /// pixels buffer
    pub pixels: Pixels<WinitWindow>,
    /// winit window
    pub winit: WinitWindow,
    /// pixel buffer size, in pixels
    pub size: Extent2<usize>,
    /// identifier passes to canvases
    pub id: CanvasId,
}

impl Window
{
    /// create a new window
    pub(crate) fn new(target: &EventLoopWindowTarget<()>, title: impl Into<String>, size: Extent2<usize>, id: CanvasId) -> Self
    {
        let logical_size = LogicalSize::new(size.w as f64, size.h as f64);

        let winit = WindowBuilder::new()
            //.with_min_inner_size(logical_size)
            .with_inner_size(logical_size)
            .with_title(title)
            .build(target)
            .unwrap();

        let pixels =
        {
            let PhysicalSize { width, height } = winit.inner_size();

            let surf_tex = SurfaceTexture::new(width, height, &winit);
            
            Pixels::new(size.w as u32, size.h as u32, surf_tex).unwrap()
        };

        Self
        {
            pixels, winit, size, id
        }
    }

    /// get the next canvas to draw to
    pub(crate) fn get_frame(&mut self) -> Canvas
    {
        Bitmap::new(self.id, self.pixels.get_frame(), self.size)
    }
}