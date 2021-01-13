use winit::event::WindowEvent;

use crate::input::InputState;
use crate::math::Vec2;

/// represents the present state of the mouse,
/// storing buttons currently down/up and those
/// pressed and released exactly this update loop.
/// also stores current mouse position/delta and
/// scroll wheel info.
#[derive(Debug, Clone, PartialEq)]
pub struct Mouse
{
    /// current mouse position
    pos: Vec2<f64>,
    /// delta mouse position
    del: Vec2<f64>,

    /// mouse button states where index is a mouse button
    btn: [InputState; 255],

    /// scroll wheel
    scroll: Vec2<f32>,
}

/// temporary mouse button
pub type MouseButton = winit::event::MouseButton;

impl Mouse
{
    /// reset the input states within `self` to prepare for
    /// a new frame
    pub(super) fn reset(&mut self)
    {
        // buttons
        for btn in self.btn.iter_mut()
        {
            *btn = match *btn
            {
                InputState::Pressed => InputState::Down,
                InputState::Released => InputState::Up,
                InputState::Down => InputState::Down,
                InputState::Up => InputState::Up,
            };
        }
        //delta
        self.del = Vec2::zero();
        // scroll
        self.scroll = Vec2::zero();
    }

    /// update the internal input states within `self` using
    /// the relevant winit-provided event
    pub(super) fn update(&mut self, event: &WindowEvent)
    {        
        match event
        {
            WindowEvent::CursorMoved { position, .. } =>
            {
                let pos = Vec2::new(position.x, position.y);

                self.del = pos - self.pos;
                self.pos = pos;
            }
            WindowEvent::MouseWheel { delta, phase, .. } =>
            {
                use winit::event::MouseScrollDelta;

                // not sure why this works... taken directly from
                // winit input helper
                const LINE_PER_PIXELS: f32 = 1.0 / 38.0;

                match delta
                {
                    MouseScrollDelta::LineDelta(x, y) =>
                    {
                        self.scroll += Vec2::new(*x, *y);
                    }
                    MouseScrollDelta::PixelDelta(dt) =>
                    {
                        self.scroll += Vec2::new(dt.x as f32, dt.y as f32) * LINE_PER_PIXELS;
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } =>
            {
                use winit::event::ElementState;

                let code = map_mouse_button(*button);

                self.btn[code] = match (state, self.btn[code])
                {
                    (ElementState::Pressed, InputState::Up) => InputState::Pressed,
                    (ElementState::Released, InputState::Down) => InputState::Released,
                    (ElementState::Pressed, _) => InputState::Down,
                    (ElementState::Released, _) => InputState::Up,
                };
            },
            _ => { }
        }
    }

    /// is the button pressed this frame or held for the duration
    /// of this frame? useful for continuous input actions,
    /// such as automatic weapons in a game.
    #[inline]
    pub fn down(&self, btn: MouseButton) -> bool
    {
        match self.btn[map_mouse_button(btn)]
        {
            InputState::Down | InputState::Pressed => true,
            _ => false
        }
    }

    /// was the button up and then pressed exactly during this frame?
    /// useful for impulsive actions, like shooting.
    #[inline]
    pub fn pressed(&self, btn: MouseButton) -> bool
    {
        self.btn[map_mouse_button(btn)] == InputState::Pressed
    }

    /// was the button down and then released exactly during this
    /// frame?
    #[inline]
    pub fn released(&self, btn: MouseButton) -> bool
    {
        self.btn[map_mouse_button(btn)] == InputState::Released
    }

    /// current mouse position, in window pixel coordinates
    #[inline]
    pub fn position(&self) -> Vec2<f64>
    {
        self.pos
    }

    /// shortcut for `Mouse::position().x`
    #[inline]
    pub fn x(&self) -> f64
    {
        self.pos.x
    }

    /// shortcut for `Mouse::position().y`
    #[inline]
    pub fn y(&self) -> f64
    {
        self.pos.y
    }

    /// delta mouse position, in window pixel coordinates
    #[inline]
    pub fn delta(&self) -> Vec2<f64>
    {
        self.del
    }

    /// shortcut for `Mouse::delta().x`
    #[inline]
    pub fn dx(&self) -> f64
    {
        self.del.x
    }

    /// shortcut for `Mouse::delta().x`
    #[inline]
    pub fn dy(&self) -> f64
    {
        self.del.y
    }

    /// [delta] scroll accumulated during this frame
    /// note: this is NOT the total scroll amount or "scroll position"
    #[inline]
    pub fn scroll(&self) -> Vec2<f32>
    {
        self.scroll
    }
}

/// utility function to map mouse buttons to a number
fn map_mouse_button(code: MouseButton) -> usize
{
    match code
    {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(num) => num as usize,
    }
}