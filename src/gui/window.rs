use egui::{ CtxRef, Window };

use crate::math::{ Vec2, Extent2 };

/// a GUI window builder, wrapping over egui's
/// [Window]. from its docs:
///
/// Builder for a floating window which can be dragged, closed, collapsed, resized and scrolled (off by default).
/// 
/// You can customize:
/// 
/// - title
/// - default, minimum, maximum and/or fixed size
/// - if the window has a scroll area (off by default)
/// - if the window can be collapsed (minimized) to just the title bar (yes, by default)
/// - if there should be a close button (none by default)
///
/// [Window]: egui::Window
pub struct Builder<'ctx, 'open>
{
    ctx: &'ctx CtxRef,
    inner: Window<'open>
}

impl<'ctx, 'open> Builder<'ctx, 'open>
{
    /// create a new GUI window builder
    pub(super) fn new(ctx: &'ctx CtxRef, title: impl Into<String>) -> Self
    {
        Self { ctx, inner: Window::new(title) }
    }

    /// Call this to add a close-button to the window title bar.
    ///
    /// * If `*open == false`, the window will not be visible.
    /// * If `*open == true`, the window will have a close button.
    /// * If the close button is pressed, `*open` will be set to `false`.
    pub fn open(mut self, open: &'open mut bool) -> Self
    {
        self.inner = self.inner.open(open);
        self
    }

    /// If `false` the window will be grayed out and non-interactive.
    pub fn enabled(mut self, enabled: bool) -> Self
    {
        self.inner = self.inner.enabled(enabled);
        self
    }

    /// Change the background color, margins, etc.
    pub fn frame(mut self, frame: egui::Frame) -> Self
    {
        self.inner = self.inner.frame(frame);
        self
    }

    /// Set minimum width of the window.
    pub fn min_width(mut self, min_width: f32) -> Self
    {
        self.inner = self.inner.min_width(min_width);
        self
    }
    /// Set minimum height of the window.
    pub fn min_height(mut self, min_height: f32) -> Self
    {
        self.inner = self.inner.min_height(min_height);
        self
    }

    /// Set current position of the window.
    /// If the window is movable it is up to you to keep track of where it moved to!
    pub fn current_pos(mut self, pos: impl Into<Vec2<f32>>) -> Self
    {
        let pos = pos.into();

        self.inner = self.inner.current_pos(egui::Pos2::new(pos.x, pos.y));
        self
    }

    /// Set initial position of the window.
    pub fn default_pos(mut self, pos: impl Into<Vec2<f32>>) -> Self
    {
        let pos = pos.into();

        self.inner = self.inner.default_pos(egui::Pos2::new(pos.x, pos.y));
        self
    }

    /// Set initial size of the window.
    pub fn default_size(mut self, siz: impl Into<Extent2<f32>>) -> Self
    {
        let siz = siz.into();

        self.inner = self.inner.default_size(egui::Vec2::new(siz.w, siz.h));
        self
    }

    /// Set initial width of the window.
    pub fn default_width(mut self, w: f32) -> Self
    {
        self.inner = self.inner.default_width(w);
        self
    }
    /// Set initial height of the window.
    pub fn default_height(mut self, h: f32) -> Self
    {
        self.inner = self.inner.default_height(h);
        self
    }

    /// Sets the window position and prevents it from being dragged around.
    pub fn fixed_pos(mut self, pos: impl Into<Vec2<f32>>) -> Self
    {
        let pos = pos.into();

        self.inner = self.inner.fixed_pos(egui::Pos2::new(pos.x, pos.y));
        self
    }

    /// Sets the window size and prevents it from being resized by dragging its edges.
    pub fn fixed_size(mut self, siz: impl Into<Extent2<f32>>) -> Self
    {
        let siz = siz.into();

        self.inner = self.inner.fixed_size(egui::Vec2::new(siz.w, siz.h));
        self
    }

    /// Can the user resize the window by dragging its edges?
    /// Note that even if you set this to `false` the window may still auto-resize.
    pub fn resizable(mut self, resizable: bool) -> Self
    {
        self.inner = self.inner.resizable(resizable);
        self
    }

    /// Can the window be collapsed by clicking on its title?
    pub fn collapsible(mut self, collapsible: bool) -> Self
    {
        self.inner = self.inner.collapsible(collapsible);
        self
    }

    /// Show title bar on top of the window?
    /// If `false`, the window will not be collapsible nor have a close-button.
    pub fn title_bar(mut self, title_bar: bool) -> Self
    {
        self.inner = self.inner.title_bar(title_bar);
        self
    }

    /// Not resizable, just takes the size of its contents.
    /// Also disabled scrolling.
    /// Text will not wrap, but will instead make your window width expand.
    pub fn auto_sized(mut self) -> Self
    {
        self.inner = self.inner.auto_sized();
        self
    }

    /// Enable/disable scrolling. `false` by default.
    pub fn scroll(mut self, scroll: bool) -> Self
    {
        self.inner = self.inner.scroll(scroll);
        self
    }

    /// build the window
    pub fn build(self, contents: impl FnOnce(&mut egui::Ui))
    {
        self.inner.show(self.ctx, contents);
    }
}