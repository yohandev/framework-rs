use framework::prelude::*;

fn main()
{
    framework::run::<SnakeGame>();
}

/// (width, height) tiles in the game
const GRID_SIZE: Vec2<i32> = v![31, 31];
/// (width, height) of a single tile, in pixels
const TILE_SIZE: Vec2<i32> = v![10, 10];
/// starting length of the snake
const START_LEN: usize = 5;

struct SnakeGame
{
    /// snake's tile positions, where `snake[0]` is
    /// the head
    snake: Vec<Vec2<i32>>,
    /// snake's direction, either ±[1, 0] or ±[0, 1]
    /// starts at [0, 0] for no direction
    dir: Vec2<i32>,
}

impl Sketch for SnakeGame
{
    fn setup(app: &mut App) -> Self
    {
        app.create_canvas("snake", (GRID_SIZE * TILE_SIZE).as_());
        app.time().frame_rate(10.0);

        Self::default()
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        // clear screen
        c.background(c!("darkslategrey"));

        // tile style
        c.fill(c!("honeydew"));
        c.no_stroke();

        // draw tiles
        for tile in &self.snake
        {
            c.rect(tile * TILE_SIZE, TILE_SIZE);
        }
    }

    fn update(&mut self, app: &mut App)
    {
        // time
        app.time().print_current_frame_rate();

        // input
        if app.keys().pressed(btn!("left"))  { self.dir = v![-1, 0]; }
        if app.keys().pressed(btn!("right")) { self.dir = v![ 1, 0]; }
        if app.keys().pressed(btn!("up"))    { self.dir = v![ 0,-1]; }
        if app.keys().pressed(btn!("down"))  { self.dir = v![ 0, 1]; }

        // no movement
        if self.dir == v![0, 0]
        {   
            return;
        }

        // move head
        self.snake[0] += self.dir;
        
        // out of grid -> game over!
        if self.snake[0].is_any_negative() || !(GRID_SIZE - self.snake[0]).are_all_positive()
        {
            *self = Self::default();

            return println!("GAME OVER!");
        }

        // move body
        for i in (1..self.snake.len()).rev()
        {
            self.snake[i] = self.snake[i - 1];
        }
    }
}

impl Default for SnakeGame
{
    fn default() -> Self
    {
        Self
        {
            // creates a snake with head in middle, and tail one
            // unit below it
            snake: [GRID_SIZE / 2; START_LEN].into(),
            // doesn't move at first
            dir: v![0, 0],
        }
    }
}