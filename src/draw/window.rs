use winit::window::{ WindowBuilder, Window as WinitWindow };
use winit::dpi::{ LogicalSize, PhysicalSize };
use winit::event_loop::EventLoopWindowTarget;

use pixels::{ Pixels, SurfaceTexture };

use crate::draw::{ Canvas, CanvasId, Bitmap };
use crate::gui::{ GuiCtx, Gui };
use crate::math::Extent2;

/// represents a window and a pixel buffer
/// attached to its swapchain
pub(crate) struct Window
{
    /// pixels buffer
    pub pixels: Pixels,
    /// winit window
    pub winit: WinitWindow,
    /// pixel buffer size, in pixels
    pub size: Extent2<usize>,
    /// this window's egui context
    pub gui: GuiCtx,
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
        let gui = GuiCtx::new(&winit, &pixels);

        Self
        {
            pixels, winit, size, gui, id
        }
    }

    /// get the next canvas to draw to
    pub(crate) fn get_frame(&mut self) -> Canvas
    {
        Bitmap::new(self.id, self.pixels.get_frame(), self.size)
    }

    /// get the canvas to draw gui to
    pub(crate) fn get_gui(&mut self) -> Gui
    {
        Gui::new(self.id, &self.gui.context())
    }

    /// resize the inner pixel and GUI framebuffer
    pub(crate) fn resize(&mut self, w: u32, h: u32)
    {
        self.pixels.resize(w, h);
        self.gui.resize(w, h);
    }
}