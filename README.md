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
use framework::math::*;
use framework::*;

fn main()
{
    run::<MySketch>();
}

struct MySketch;

impl App for MySketch
{
    fn render(&mut self, frame: &mut Frame)
    {
        frame.clear(Rgba::black());
    }

    fn update(&mut self, time: &Time)
    {
        println!("FPS: {:.1}", 1.0 / time.dt())
    }
}
```

## Examples
This library was designed for personal use, and may be feature incomplete. Still, you can see my other projects that use `framework-rs` as reference:
- [software-rasterizer](https://github.com/yohandev/software-rasterizer)
- [timbre-shift](https://github.com/yohandev/timbre-shift)