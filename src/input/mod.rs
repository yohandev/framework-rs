mod mouse;
mod keys;

pub use self::mouse::Mouse;
pub use self::keys::Keys;

/// enumeration to cache the state of input keys and buttons
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

// TODO use own types for keycode and mouse button
pub use winit::event::{ Event };

/// stores raw keyboard, mouse, and [TODO] controller input.
/// It caches physical buttons that are held or up, as well as
/// buttons pressed or released during the duration of this frame.
pub struct Input
{
    /// mouse input
    mouse: Mouse,
    /// keyboard input
    keys: Keys,
}

impl Input
{
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