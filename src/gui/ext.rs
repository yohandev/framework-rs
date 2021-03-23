use std::ops::RangeInclusive;

use crate::math::Rgb;

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
    fn slider<'b>(&'b mut self, value: &'a mut T, range: RangeInclusive<T>) -> SliderBuilder<'a, 'b>;

    // /// create an [egui::DragValue]. from its docs:
    // /// A numeric value that you can change by dragging the number. More compact than a [`Slider`].
    // ///
    // /// ```
    // /// ui.add(egui::DragValue::f32(&mut my_f32).speed(0.1));
    // /// ```
    // ///
    // /// [egui::DragValue]: egui::DragValue
    // fn drag_value<'b>(&'b mut self, value: &'a mut T) -> Builder<'b, egui::DragValue<'a>>;
}

/// a builder for a GUI slider, that adds the created element
/// on drop
pub struct SliderBuilder<'val, 'ctx>
{
    inner: egui::Slider<'val>,
    ctx: &'ctx mut egui::Ui,
}

macro_rules! copy_fn
{
    ($(
        $(#[$meta:meta])*
        pub fn $name:ident(mut self, $($arg_ident:ident: $arg_ty:ty),*) -> Self
    );*$(;)?) =>
    {
        $(
            $(#[$meta])*
            pub fn $name(mut self, $($arg_ident: $arg_ty),*) -> Self
            {
                self.inner = self.inner.$name($($arg_ident),*);
                self
            }
        )*
    };
}

impl<'val, 'ctx> SliderBuilder<'val, 'ctx>
{
    /// add this slider to the GUI, and get the interaction response
    pub fn build(self) -> egui::Response
    {
        self.ctx.add(self.inner)
    }

    copy_fn!(
        /// Show a prefix before the number, e.g. "x: "
        pub fn show_value(mut self, show_value: bool) -> Self;
        /// Add a suffix to the number, this can be e.g. a unit ("°" or " m")
        pub fn suffix(mut self, suffix: impl ToString) -> Self;
        /// Show a text next to the slider (e.g. explaining what the slider controls).
        pub fn text(mut self, text: impl Into<String>) -> Self;
        /// Make this a logarithmic slider.
        /// This is great for when the slider spans a huge range,
        /// e.g. from one to a million.
        /// The default is OFF.
        pub fn logarithmic(mut self, logarithmic: bool) -> Self;
        /// For logarithmic sliders that includes zero:
        /// what is the smallest positive value you want to be able to select?
        /// The default is `1` for integer sliders and `1e-6` for real sliders.
        pub fn smallest_positive(mut self, smallest_positive: f64) -> Self;
        /// For logarithmic sliders, the largest positive value we are interested in
        /// before the slider switches to `INFINITY`, if that is the higher end.
        /// Default: INFINITY.
        pub fn largest_finite(mut self, largest_finite: f64) -> Self;
        /// If set to `true`, all incoming and outgoing values will be clamped to the slider range.
        /// Default: `false`.
        pub fn clamp_to_range(mut self, clamp_to_range: bool) -> Self;
        /// Turn smart aim on/off. Default is ON.
        /// There is almost no point in turning this off.
        pub fn smart_aim(mut self, smart_aim: bool) -> Self;
        // TODO: we should also have a "min precision".
        /// Set a minimum number of decimals to display.
        /// Normally you don't need to pick a precision, as the slider will intelligently pick a precision for you.
        /// Regardless of precision the slider will use "smart aim" to help the user select nice, round values.
        pub fn min_decimals(mut self, min_decimals: usize) -> Self;
        // TODO: we should also have a "max precision".
        /// Set a maximum number of decimals to display.
        /// Values will also be rounded to this number of decimals.
        /// Normally you don't need to pick a precision, as the slider will intelligently pick a precision for you.
        /// Regardless of precision the slider will use "smart aim" to help the user select nice, round values.
        pub fn max_decimals(mut self, max_decimals: usize) -> Self;
        /// Set an exact number of decimals to display.
        /// Values will also be rounded to this number of decimals.
        /// Normally you don't need to pick a precision, as the slider will intelligently pick a precision for you.
        /// Regardless of precision the slider will use "smart aim" to help the user select nice, round values.
        pub fn fixed_decimals(mut self, num_decimals: usize) -> Self;
        /// Helper: equivalent to `self.precision(0).smallest_positive(1.0)`.
        /// If you use one of the integer constructors (e.g. `Slider::i32`) this is called for you,
        /// but if you want to have a slider for picking integer values in an `Slider::f64`, use this.
        pub fn integer(mut self,) -> Self;
    );

    pub fn text_color(mut self, c: Rgb<u8>) -> Self
    {
        self.inner = self.inner.text_color(egui::Color32::from_rgb(c.r, c.g, c.b));
        self
    }
}

macro_rules! impl_ext
{
    ($typ:tt) =>
    {
        impl<'a> UiExt<'a, $typ> for egui::Ui
        {
            fn slider<'b>(&'b mut self, value: &'a mut $typ, range: RangeInclusive<$typ>) -> SliderBuilder<'a, 'b>
            {
                SliderBuilder
                {
                    inner: egui::Slider::<'a>::$typ(value, range),
                    ctx: self,
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