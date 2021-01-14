use std::collections::HashMap;

use winit::event_loop::{ ControlFlow, EventLoopWindowTarget as WindowTarget };
use winit::window::WindowId;
use winit::event::Event;

use rand::prelude::ThreadRng;

use crate::input::{ Input, Mouse, Keys, Time, ProcessedEvent };
use crate::draw::{ CanvasId, Window };
use crate::math::Extent2;
use crate::Sketch;

/// handle to the application running this `Sketch`.
/// this is how `Canvas`es are created or input and
/// time is accessed.
#[derive(Debug)]
pub struct App
{
    /// input state
    input: Input,
    /// time state
    time: Time,

    /// windows currently open
    windows: Windows,
    /// window creation requests
    requests: Vec<(CanvasId, String, Extent2<usize>)>,
    /// next window request ID
    next: CanvasId,

    /// extra utility: random
    rand: ThreadRng,
}

/// double-key'd hashmap of `CanvasId` and `WindowId`s
/// pointing to existing and open `Window`s
#[derive(Debug, Default)]
pub(crate) struct Windows
{
    /// maps canvas id to window id
    id: HashMap<CanvasId, WindowId>,
    /// maps windows id to windows currently open
    win: HashMap::<WindowId, Window>
}

impl App
{
    /// create a new app
    pub(super) fn new() -> Self
    {
        Self
        {
            input: Input::new(),
            time: Time::new(),
            windows: Windows::default(),
            requests: Vec::new(),
            next: CanvasId::zero(),
            rand: ThreadRng::default(),
        }
    }

    /// process an incoming winit event and interact with
    /// `Sketch`, inner `Time`, `Input`, and windows accordingly.
    pub(super) fn process_event(
        &mut self,
        sketch: &mut impl Sketch,
        event: Event<()>,
        window_target: &WindowTarget<()>,
        control_flow: &mut ControlFlow
    )
    {
        match self.input.process(event, &self.windows)
        {
            // notify `Pixels` of the window resize
            ProcessedEvent::WindowResized(id, (w, h)) =>
            {
                if let Some(window) = self.windows.get_mut(&id)
                {
                    window.pixels.resize(w, h);
                }
            }
            // close window and quit if needed
            ProcessedEvent::WindowClose(id) =>
            {
                // stop keeping track of and drop window
                self.windows.remove2(&id);

                // quit?
                if self.windows.is_empty()
                {
                    return *control_flow = ControlFlow::Exit;
                }
            }
            // render to the given window
            ProcessedEvent::ShouldRender(id) =>
            {
                // get the frame
                if let Some(window) = self.windows.get_mut(&id)
                {
                    // update buffer
                    sketch.draw(&mut window.get_frame());

                    // render
                    if window.pixels.render().is_err()
                    {
                        return *control_flow = ControlFlow::Exit;
                    }
                }
            }
            // update the Sketch state and request new events
            ProcessedEvent::ShouldUpdate =>
            {
                // update time
                if self.time.update()
                {
                    // update state if time says it's time to
                    // update
                    sketch.update(self);

                    // process requests
                    self.process_requests(window_target);

                    // input has been used by update
                    self.input.reset = true;
                }
                
                // request a redraw on each window
                for window in self.windows.iter_mut()
                {
                    window.winit.request_redraw();
                }
            }
            ProcessedEvent::None => { }
        }
    }

    /// process window creation requests. this should be
    /// called right after the `App` has been mutuably passed
    /// to the `Sketch`
    pub(super) fn process_requests(&mut self, target: &WindowTarget<()>)
    {
        while let Some((id, title, size)) = self.requests.pop()
        {
            self.windows.insert(id, Window::new(target, title, size, id));
        }
    }

    /// get the current `Time` information for this
    /// frame
    #[inline]
    pub fn time(&mut self) -> &mut Time
    {
        &mut self.time
    }

    /// get the current state of the keyboard for this
    /// frame
    #[inline]
    pub fn keys(&self) -> &Keys
    {
        self.input.keys()
    }

    /// get the current state of the mouse for this
    /// frame
    #[inline]
    pub fn mouse(&self) -> &Mouse
    {
        self.input.mouse()
    }

    /// create a new `Canvas`, and effectively a window, to
    /// draw to. returns that `Canvas`' `CanvasId` to reference
    /// back to it later on. note that the window won't actually
    /// be instantly created. instead, calls to `create_canvas`
    /// are polled right after the enclosing function goes out of
    /// scope. 
    pub fn create_canvas(&mut self, title: impl Into<String>, size: impl Into<Extent2<usize>>) -> CanvasId
    {
        let id = self.next;

        self.next = self.next.next();
        self.requests.push((id, title.into(), size.into()));

        id
    }

    /// destroys a `Canvas` given its ID if it exists, or
    /// does nothing if it doesn't
    pub fn destroy_canvas(&mut self, id: CanvasId)
    {
        self.windows.remove(&id);
    }

    /// get this app's random number generator
    pub fn random(&mut self) -> &mut ThreadRng
    {
        &mut self.rand
    }
}

impl Windows
{
    /// get a window given its `WindowId`
    pub fn get(&self, id: &WindowId) -> Option<&Window>
    {
        self.win.get(id)
    }

    /// mutably get a window given its `WindowId`
    pub fn get_mut(&mut self, id: &WindowId) -> Option<&mut Window>
    {
        self.win.get_mut(id)
    }

    /// insert a window given its `CanvasId`
    pub fn insert(&mut self, id: CanvasId, window: Window)
    {
        self.id.insert(id, window.winit.id());
        self.win.insert(window.winit.id(), window);
    }

    /// remove a window given its `CanvasId`
    pub fn remove(&mut self, id: &CanvasId) 
    {
        if let Some(id) = self.id.remove(&id)
        {
            self.win.remove(&id);
        }
    }

    /// remove a window given its `WindowId`
    pub fn remove2(&mut self, id: &WindowId) 
    {
        if let Some(Window { id, .. }) = self.win.remove(&id)
        {
            self.id.remove(&id);
        }
    }

    /// is `self` empty of `Window`s?
    pub fn is_empty(&mut self) -> bool
    {
        debug_assert_eq!(self.win.is_empty(), self.id.is_empty());

        self.win.is_empty()
    }

    /// mutuably iterate over all the windows in 
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Window>
    {
        self.win.values_mut()
    }
}