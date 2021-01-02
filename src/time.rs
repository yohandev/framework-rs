use std::time::*;

/// number of samples used to determine Time::fps()
const FPS_SAMPLE_SIZE: u64 = 200;

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
    /// current tick, where each frame is 1 tick and the first
    /// update call is tick 1
    tick: u64,
    /// (sums of delta time in the current fps sample, fps)
    fps: (f32, f32),
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

            tick: 0,
            fps: (0.0, 0.0),
        }
    }

    /// updates this instance of Time and returns itself
    /// to pass to App's update
    pub(crate) fn update(&mut self) -> &Self
    {
        self.tick += 1;

        self.delta = self.start.elapsed();
        self.total += self.delta;
        self.start = Instant::now();

        self.fps.0 += self.dt();
        if self.fps_was_updated()
        {
            self.fps.1 = FPS_SAMPLE_SIZE as f32 / self.fps.0;
            self.fps.0 = 0.0;
        }

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

    /// current tick, where each frame is 1 tick and the first
    /// update call is tick 1
    pub fn tick(&self) -> u64
    {
        self.tick
    }

    /// get the frames per second, calculated with a sample size
    /// of n frames per second every n frames
    ///
    /// currently, n = 200
    pub fn fps(&self) -> f32
    {
        self.fps.1
    }

    /// since `Time::fps()` isn't calculated every frame, was it
    /// calculated this frame?
    pub fn fps_was_updated(&self) -> bool
    {
        self.tick % FPS_SAMPLE_SIZE == 0
    }
}