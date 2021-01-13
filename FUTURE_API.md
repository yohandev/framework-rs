# future API
## something that's closer to p5.js...
```rust
impl Sketch for Foo
{
    // the only required fn
    fn setup(app: &mut App) -> Self
    {
        // gives back a canvas id. useful when
        // there are multiple canvases
        app.create_canvas(720, 400);

        // load audio. the first app.audio() call
        // initializes audio subsystem
        let song = app.audio().load("piano.ogg");

        Foo { song }
    }

    // only called if a canvas exists. then, it is
    // called for every existing canvas on every
    // window redraw event regardless of whether
    // this method is implemented.
    fn draw(&mut self, c: &mut Canvas)
    {
        c.background(Rgba::BLACK);

        c.stroke(c!("blue"));
        c.line(v![0, 0], v![20, 40]);

        c.fill(c![0x23, 0xff, 0x12, 0xff]);
        c.triangle(v![20, 50], v![1, 2], v![50, 80]);
        c.ellipse(v![70, 70], v![10, 99]);
    }

    // update the sketch state [and app]. called
    // on every main events clear event.
    fn update(&mut self, app: &mut App)
    {
        app.time().dt();            // delta time
        app.keys().pressed(Key::A); // pressed this frame?
        app.keys().down(Key::B);    // pressed at all?
        app.mouse().down(Mouse::R); // same deal as keys
        app.mouse().x();            // position in pixels

        if app.keys().pressed(Key::Space)
        {
            app.audio().play(&self.song);
        }
        else if app.keys().pressed(Key::Backspace)
        {
            app.audio().pause(&self.song);
        }
        else if app.keys().pressed(Key::P)
        {
            app.audio().resume(&self.song);
        }
    }

    // called exactly when a key is pressed.
    // alternatively, just check these in update
    // using `app.keys().pressed(/**/)` to check if
    // that key has been pressed this frame.
    fn key_pressed(&mut self, key: Key)
    {
        // *might not make it to final api*
    }
}
```