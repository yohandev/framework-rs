use std::ops::RangeInclusive;

/// extension trait for [egui::Ui] to conveniently make UI elements that would
/// normally require even more imports from `egui`
///
/// [egui::Ui]: egui::Ui
pub trait UiExt<'a, T>
{
    /// create an [egui::Slider]. from its docs:
    ///
    /// Control a number by a horizontal slider.
    /// 
    /// The slider range defines the values you get when pulling the slider
    /// to the far edges. By default, the slider can still show values outside
    /// this range, and still allows users to enter values outside the range
    /// by clicking the slider value and editing it.
    ///
    /// The range can include any numbers, and go from low-to-high or from high-to-low.
    ///
    /// The slider consists of three parts: a horizontal slider, a value display, and
    /// an optional text. The user can click the value display to edit its value. It
    /// can be turned off with .show_value(false).
    ///
    /// [egui::Slider]: egui::Slider
    fn slider<'b>(&'b mut self, value: &'a mut T, range: RangeInclusive<T>) -> Builder<'b, egui::Slider<'a>>;

    /// create an [egui::DragValue]. from its docs:
    /// A numeric value that you can change by dragging the number. More compact than a [`Slider`].
    ///
    /// ```
    /// ui.add(egui::DragValue::f32(&mut my_f32).speed(0.1));
    /// ```
    ///
    /// [egui::DragValue]: egui::DragValue
    fn drag_value<'b>(&'b mut self, value: &'a mut T) -> Builder<'b, egui::DragValue<'a>>;
}

/// a builder for GUI elements used by [UiExt]
///
/// [UiExt]: crate::ui::UiExt
pub struct Builder<'a, T: egui::Widget>
{
    /// element builder provided by egui
    ele: T,
    /// temporary reference to UI while 
    ui: &'a mut egui::Ui,
}

impl<'a, T: egui::Widget> Builder<'a, T>
{
    /// build and add the widget
    pub fn build(self) -> egui::Response
    {
        self.ui.add(self.ele)
    }
}

macro_rules! impl_ext
{
    ($typ:tt) =>
    {
        impl<'a> UiExt<'a, $typ> for egui::Ui
        {
            fn slider<'b>(&'b mut self, value: &'a mut $typ, range: RangeInclusive<$typ>) -> Builder<'b, egui::Slider<'a>>
            {
                Builder
                {
                    ele: egui::Slider::<'a>::$typ(value, range),
                    ui: self,
                }
            }

            fn drag_value<'b>(&'b mut self, value: &'a mut $typ) -> Builder<'b, egui::DragValue<'a>>
            {
                Builder
                {
                    ele: egui::DragValue::<'a>::$typ(value),
                    ui: self,
                }
            }
        }
    };
    ($($typ:tt),*) =>
    {
        $(impl_ext!($typ);)+
    }
}

impl_ext!(f32, f64, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);