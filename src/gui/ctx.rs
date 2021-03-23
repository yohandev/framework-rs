use egui_wgpu_backend::{ RenderPass, ScreenDescriptor };
use egui_winit_platform::{ Platform, PlatformDescriptor };

use winit::window::{ Window as WinitWindow };
use winit::event::Event;

use pixels::{ Pixels, PixelsContext, wgpu };
use egui::CtxRef;

use crate::input::Time;

/// gui manager for one [Canvas]
///
/// [Canvas]: crate::draw::Canvas
pub(crate) struct GuiCtx
{
    /// winit platform info
    platform: Platform,
    /// info about screen from winit
    screen_desc: ScreenDescriptor,
    /// wgpu render pass to draw egui
    rpass: RenderPass,
}

impl GuiCtx
{
    /// create a new GUI context for a given window
    pub(crate) fn new(winit: &WinitWindow, pixels: &Pixels) -> Self
    {
        let size = winit.inner_size();
        let scale = winit.scale_factor();

        let platform = Platform::new(PlatformDescriptor
        {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: scale,
            font_definitions: Default::default(),
            style: Default::default(),
        });
        let screen_desc = ScreenDescriptor
        {
            physical_width: size.width,
            physical_height: size.height,
            scale_factor: scale as f32,
        };
        let rpass = RenderPass::new(pixels.device(), pixels::wgpu::TextureFormat::Bgra8UnormSrgb);

        Self { platform, screen_desc, rpass }
    }

    /// prepare a new GUI frame. this should be called before
    /// `render` and any UI drawing logic
    pub(crate) fn prepare(&mut self, time: &Time)
    {
        // update time for animations
        self.platform.update_time(time.elapsed().as_secs_f64());
        // begin GUI frame
        self.platform.begin_frame();
    }

    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        pixels: &PixelsContext
    )
    {
        // get paint commands
        let (_, cmds) = self.platform.end_frame();
        // paint commands to meshes
        let meshes = self.platform
            .context()
            .tessellate(cmds);

        // upload all resources to the GPU.
        self.rpass.update_texture(
            &pixels.device,
            &pixels.queue,
            &*self.platform.context().texture(),
        );
        self.rpass
            .update_user_textures(&pixels.device, &pixels.queue);
        self.rpass.update_buffers(
            &pixels.device,
            &pixels.queue,
            &meshes,
            &self.screen_desc,
        );

        // record all render passes.
        self.rpass.execute(encoder, target, &meshes, &self.screen_desc, None);
    }

    /// process winit event
    pub(crate) fn process(&mut self, event: &Event<()>)
    {
        self.platform.handle_event(event);
    }

    /// update the egui screen descriptor with the new size
    pub(crate) fn resize(&mut self, w: u32, h: u32)
    {
        self.screen_desc.physical_width = w;
        self.screen_desc.physical_height = h;
    }

    /// get the egui context reference
    pub(crate) fn context(&self) -> CtxRef
    {
        self.platform.context()
    }
}