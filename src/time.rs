use std::time::*;

/// represents a snapshot of time in the app
#[derive(Debug, Clone)]
pub struct Time
{
    /// delta time between the beginnings of the last frame and
    /// of this frame
    delta: Duration,
    /// total time since the app has started
    total: Duration,
    /// start time of this frame
    start: Instant,
}

impl Time
{
    /// create the default Time instance, initialized to the
    /// current time
    pub(crate) fn new() -> Self
    {
        Self
        {
            delta: Duration::default(),
            total: Duration::default(),
            start: Instant::now(),
        }
    }

    /// updates this instance of Time and returns itself
    /// to pass to App's update
    pub(crate) fn update(&mut self) -> &Self
    {
        self.delta = self.start.elapsed();
        self.total += self.delta;
        self.start = Instant::now();

        self
    }

    /// delta time between the beginnings of the last frame and
    /// of this frame    
    pub fn delta(&self) -> Duration
    {
        self.delta
    }

    /// delta seconds between the beginnings of the last frame and
    /// of this frame
    ///
    /// this is the exact same as doing:
    /// ```
    /// time.delta().as_secs_f32()
    /// ```
    pub fn dt(&self) -> f32
    {
        self.delta.as_secs_f32()
    }

    /// total time since the app has started
    pub fn elapsed(&self) -> Duration
    {
        self.total
    }

    /// start time of this frame
    pub fn start(&self) -> Instant
    {
        self.start
    }
}