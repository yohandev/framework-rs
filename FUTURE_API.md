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
        // use macro...
        background!(c, BLACK);
        // ...or type it out manually:
        c.background(Rgba::BLACK);

        // use macros...
        stroke!(c, BLUE);
        line!(c, [0, 0], [20, 40]);
        // ...or type it out manually:
        c.stroke(Rgba::BLUE);
        c.line(Vec2::new(0, 0), Vec2::new(20, 40));

        // other macros...
        fill!(c, [0x23, 0xff, 0x12, 0xff]);
        triangle!(c, [20, 50], [1, 2], [50, 80]);
        ellipse!(c, [70, 70], [10, 99]);
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