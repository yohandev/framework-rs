use egui::CtxRef;

use crate::draw::CanvasId;
use crate::gui::window;

/// wrapper around a [Canvas] that provides GUI
/// methods using [egui]
///
/// [Canvas]: crate::draw::Canvas
/// [egui]: egui
pub struct Gui
{
    /// underlying canvas
    id: CanvasId,
    /// egui context ref
    inner: CtxRef,
}

impl Gui
{
    /// create a new canvas GUI
    pub(crate) fn new(id: CanvasId, inner: &CtxRef) -> Self
    {
        Self { id, inner: inner.clone() }
    }

    /// get the ID of the underlying [Canvas] for this
    /// gui
    ///
    /// [Canvas]: crate::draw::Canvas
    pub fn id(&self) -> CanvasId
    {
        self.id
    }

    /// create a new GUI window. this returns a builder, which
    /// won't do anything until its `build` method is called:
    /// ```
    /// gui
    ///     .window("my window!")
    ///     .resizeable(true)
    ///     .default_size(v![200, 400])
    ///     .build(|ui|
    ///     {
    ///         ui.label("hello!");
    ///     });
    /// ```
    pub fn window<'a>(&'a self, title: impl Into<String>) -> window::Builder
    {
        window::Builder::new(&self.inner, title)
    }
}