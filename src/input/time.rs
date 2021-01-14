use std::time::*;

/// number of samples used to determine Time::fps()
const FPS_SAMPLE_SIZE: u64 = 200;

/// represents a snapshot of time in the app
#[derive(Debug, Clone)]
pub struct Time
{
    /// frame rate artificial limit, if any
    pub limit: Option<Duration>,
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
            limit: None,

            delta: Duration::default(),
            total: Duration::default(),
            start: Instant::now(),

            tick: 0,
            fps: (0.0, 0.0),
        }
    }

    /// updates this instance of Time and returns whether
    /// update should be called
    pub(crate) fn update(&mut self) -> bool
    {
        // time since last frame
        self.delta = self.start.elapsed();

        // has artificial frame rate limit?
        if let Some(limit) = self.limit
        {
            // update not ready to be called
            if self.delta < limit
            {
                return false;
            }
        }

        // update
        self.tick += 1;
        self.total += self.delta;
        self.start = Instant::now();

        // fps monitoring
        self.fps.0 += self.dt();
        if self.fps_was_updated()
        {
            self.fps.1 = FPS_SAMPLE_SIZE as f32 / self.fps.0;
            self.fps.0 = 0.0;
        }

        return true;
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

    /// sets the target frame rate. note that this rate may not
    /// be reached(if the hardware isn't powerful enough) and is
    /// likely to be innacurate. Use `Time::delta()` for time
    /// sensitive cases
    ///
    /// this is a shorthand for:
    /// ```
    /// app.time().limit = Some(Duration::from_secs_f32(1.0 / rate));
    /// ```
    pub fn frame_rate(&mut self, rate: f32)
    {
        self.limit = Some(Duration::from_secs_f32(1.0 / rate));
    }

    /// get the frames per second, calculated with a sample size
    /// of n frames per second every n frames
    ///
    /// currently, n = 200
    pub fn current_frame_rate(&self) -> f32
    {
        self.fps.1
    }

    /// utility function that prints the current frames per second
    /// if and only if the frames per second has been recalculated
    /// this frame. it's useful to call this function once every frame
    /// to get a [non-spammy] FPS report of your `Sketch`
    pub fn print_current_frame_rate(&self)
    {
        if self.fps_was_updated()
        {
            println!("FPS: {:.2}", self.current_frame_rate());
        }
    }

    /// since `Time::currentFrameRate()` isn't calculated every frame,
    /// was it calculated this frame?
    fn fps_was_updated(&self) -> bool
    {
        self.tick % FPS_SAMPLE_SIZE == 0
    }
}