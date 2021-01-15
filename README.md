# framework-rs
A rust framework for sketches and experiments

## Usage
add this to your `Cargo.toml`
```toml
[dependencies]
framework = { git = "https://github.com/yohandev/framework-rs" }
```
...then:
```rust
use framework::prelude::*;

fn main()
{
    framework::run::<Foo>();
}

struct Foo;

impl Sketch for Foo
{
    fn setup(app: &mut App) -> Self
    {
        app.create_canvas("Foo", (400, 300));

        Foo
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // clear background to blue
        c.background(c!("royalblue"));

        // set the stroke and colour using CSS colour codes.
        c.stroke(c!("teal"));
        c.fill(c!("purple"));

        // draw some shape
        c.triangle(v![0, 0], v![10, 10], v![30, 20]);
    }
}
```

## Examples
This library was designed for personal use, and may be feature incomplete. Still, you can see my other projects that use `framework-rs` as reference:
- [software-rasterizer](https://github.com/yohandev/software-rasterizer)
- [timbre-shift](https://github.com/yohandev/timbre-shift)