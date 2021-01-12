// TODO use own types for keycode and mouse button
pub use winit::event::{ Event, VirtualKeyCode as KeyCode, MouseButton };

/// stores raw keyboard, mouse, and [TODO] controller input.
/// It caches physical buttons that are held or up, as well as
/// buttons pressed or released during the duration of this frame.
pub struct Input
{
    cursor: [f64; 2],        // actual mouse position
    delta: [f64; 2],         // delta mouse position
    scroll: [f32; 2],        // scroll wheel

    keys: [InputState; 255], // up, down, pressed, and released data
    btns: [InputState; 255], // same as keys, but for mouse buttons
}

/// enumeration to cache the state of input keys and buttons, used by
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
enum InputState
{
    /// input is not held for the duration of this frame
    Up = 0,
    /// input is held for the duration of this frame
    Down = 1,
    /// input started being pressed during this frame
    Pressed = 2,
    /// input stopped being pressed during this frame
    Released = 3,
}

impl Input
{
    pub(crate) fn new() -> Self
    {
        Self
        {
            cursor: [0.0; 2],
            scroll: [0.0; 2],
            delta: [0.0; 2],

            keys: [InputState::Up; 255],
            btns: [InputState::Up; 255],
        }
    }

    /// is the key pressed this frame or held for the duration
    /// of this frame? useful for continuous input actions,
    /// such as character movement.
    pub fn key_down(&self, code: KeyCode) -> bool
    {
        match self.keys[code as usize]
        {
            InputState::Down => true,
            InputState::Pressed => true,
            _ => false
        }
    }

    /// was the key up and then pressed exactly during this frame?
    /// useful for impulsive actions, like jumping.
    pub fn key_pressed(&self, code: KeyCode) -> bool
    {
        self.keys[code as usize] == InputState::Pressed
    }

    /// was the key down and then released exactly druing this
    /// frame?
    pub fn key_released(&self, code: KeyCode) -> bool
    {
        self.keys[code as usize] == InputState::Released
    }

    /// opposite of Input::key_down()
    pub fn key_up(&self, code: KeyCode) -> bool
    {
        !self.key_down(code)
    }

    /// is the button pressed this frame or held for the duration
    /// of this frame? useful for continuous input actions,
    /// such as automatic weapons in a game.
    pub fn button_down(&self, code: MouseButton) -> bool
    {
        match self.btns[map_mouse_button(&code)]
        {
            InputState::Down => true,
            InputState::Pressed => true,
            _ => false
        }
    }

    /// was the button up and then pressed exactly during this frame?
    /// useful for impulsive actions, like shooting.
    pub fn button_pressed(&self, code: MouseButton) -> bool
    {
        self.btns[map_mouse_button(&code)] == InputState::Pressed
    }

    /// was the button down and then released exactly druing this
    /// frame?
    pub fn button_released(&self, code: MouseButton) -> bool
    {
        self.btns[map_mouse_button(&code)] == InputState::Released
    }

    /// opposite of Input::button_down()
    pub fn button_up(&self, code: MouseButton) -> bool
    {
        !self.button_down(code)
    }

    /// current mouse position, in window pixel coordinates
    pub fn cursor(&self) -> &[f64; 2]
    {
        &self.cursor
    }

    /// x position of the current mouse position, in window pixel
    /// coordinates.
    pub fn cursor_x(&self) -> f64
    {
        self.cursor[0]
    }

    /// y position of the current mouse position, in window pixel
    /// coordinates.
    pub fn cursor_y(&self) -> f64
    {
        self.cursor[1]
    }

    /// delta mouse position, in window pixel coordinates
    pub fn delta(&self) -> &[f64; 2]
    {
        &self.delta
    }

    /// x dimension of the delta mouse, in window pixel
    /// coordinates
    pub fn dx(&self) -> f64
    {
        self.delta[0]
    }

    /// y dimension of the delta mouse, in window pixel
    /// coordinates
    pub fn dy(&self) -> f64
    {
        self.delta[1]
    }

    /// delta scroll during this frame. if you need the total scroll
    /// "position, " for some reason, you'll have to accumulate this
    /// value each frame.
    pub fn scroll(&self) -> &[f32; 2]
    {
        &self.scroll
    }

    /// delta scroll in the horizontal(x-axis) during this frame.
    pub fn scroll_x(&self) -> f32
    {
        self.scroll[0]
    }

    /// delta scroll in the vertical(y-axis) during this frame.
    pub fn scroll_y(&self) -> f32
    {
        self.scroll[1]
    }

    /// simulates an axis given two keys, returning 0 if both
    /// or neither are pressed.
    pub fn axis(&self, neg: KeyCode, pos: KeyCode) -> f32
    {
        let mut val = 0.0;

        if self.key_down(neg) { val -= 1.0; }
        if self.key_down(pos) { val += 1.0; }

        val
    }

    /// get the state of the key for this frame. it's preferred
    /// to use Input::key_up(), Input::key_down(), etc.
    pub fn key_state(&self, code: KeyCode) -> InputState
    {
        self.keys[code as usize]
    }

    pub(crate) fn update(&mut self, evt: Event<()>)
    {
        if let Event::NewEvents(_) = evt
        {
            // reset keyboard keys
            for key in self.keys.iter_mut()
            {
                *key = match *key
                {
                    InputState::Pressed => InputState::Down,
                    InputState::Released => InputState::Up,
                    InputState::Down => InputState::Down,
                    InputState::Up => InputState::Up,
                };
            }

            // reset mouse buttons
            for btn in self.btns.iter_mut()
            {
                *btn = match *btn
                {
                    InputState::Pressed => InputState::Down,
                    InputState::Released => InputState::Up,
                    InputState::Down => InputState::Down,
                    InputState::Up => InputState::Up,
                };
            }

            // reset mouse scroll
            self.scroll[0] = 0.0;
            self.scroll[1] = 0.0;

            // reset mouse delta
            self.delta[0] = 0.0;
            self.delta[1] = 0.0;
        }
        if let Event::WindowEvent { event, ..} = evt
        {
            match event
            {
                winit::event::WindowEvent::KeyboardInput { input, .. } =>
                {
                    if let Some(code) = input.virtual_keycode
                    {
                        let code = code as usize;

                        match input.state
                        {
                            winit::event::ElementState::Pressed =>
                            {
                                self.keys[code] = if self.keys[code] == InputState::Up
                                {
                                    InputState::Pressed
                                }
                                else
                                {
                                    InputState::Down
                                };
                            }
                            winit::event::ElementState::Released =>
                            {
                                self.keys[code] = if self.keys[code] == InputState::Down
                                {
                                    InputState::Released
                                }
                                else
                                {
                                    InputState::Up
                                };
                            }
                        }
                    }
                }
                winit::event::WindowEvent::CursorMoved { position, .. } =>
                {
                    self.delta[0] = position.x - self.cursor[0];
                    self.delta[1] = position.y - self.cursor[1];

                    self.cursor[0] = position.x;
                    self.cursor[1] = position.y;
                }
                winit::event::WindowEvent::MouseWheel { delta, .. } =>
                {
                    const PIXELS_PER_LINE: f32 = 38.0;

                    match delta
                    {
                        winit::event::MouseScrollDelta::LineDelta(x, y) =>
                        {
                            self.scroll[0] += x;
                            self.scroll[1] += y;
                        }
                        winit::event::MouseScrollDelta::PixelDelta(dt) =>
                        {
                            self.scroll[0] += dt.x as f32 / PIXELS_PER_LINE;
                            self.scroll[1] += dt.y as f32 / PIXELS_PER_LINE;
                        }
                    }
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } =>
                {
                    let code = map_mouse_button(&button);

                    match state
                    {
                        winit::event::ElementState::Pressed =>
                        {
                            self.btns[code] = if self.btns[code] == InputState::Up
                            {
                                InputState::Pressed
                            }
                            else
                            {
                                InputState::Down
                            };
                        }
                        winit::event::ElementState::Released =>
                        {
                            self.btns[code] = if self.btns[code] == InputState::Down
                            {
                                InputState::Released
                            }
                            else
                            {
                                InputState::Up 
                            };
                        }
                    }
                },
                _ => {}
            }
        }
    }
}

/// utility function to map mouse buttons to a number
fn map_mouse_button(code: &MouseButton) -> usize
{
    match code
    {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(byte) => *byte as usize,
    }
}