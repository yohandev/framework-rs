use framework::prelude::*;
use framework::math::Wrap;

fn main()
{
    framework::run::<SnakeGame>();
}

/// (width, height) tiles in the game
const GRID_SIZE: Vec2<i32> = v![30, 30];
/// (width, height) of a single tile, in pixels
const TILE_SIZE: Vec2<i32> = v![10, 10];
/// starting length of the snake
const START_LEN: i32 = 5;

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

        Self
        {
            // creates a snake with head in middle, and tail one
            // unit below it
            snake: (0..START_LEN).map(|n| GRID_SIZE / 2 + v![0, n]).collect(),
            dir: v![0, 0],
        }
    }

    fn draw(&mut self, c: &mut Canvas)
    {
        c.background(c!("darkslategrey"));

        // draw tiles
        c.fill(c!("honeydew"));
        c.no_stroke();

        for tile in &self.snake
        {
            c.rect(tile * TILE_SIZE, TILE_SIZE);
        }
    }

    fn update(&mut self, app: &mut App)
    {
        // input
        if app.keys().pressed(KeyCode::Left)  { self.dir = v![-1, 0]; }
        if app.keys().pressed(KeyCode::Right) { self.dir = v![ 1, 0]; }
        if app.keys().pressed(KeyCode::Up)    { self.dir = v![0, -1]; }
        if app.keys().pressed(KeyCode::Down)  { self.dir = v![ 0, 1]; }

        // no movement
        if self.dir == v![0, 0]
        {   
            return;
        }

        // move head
        self.snake[0] += self.dir;
        
        // out of grid -> wrap around
        self.snake[0] = self.snake[0].wrapped(GRID_SIZE);

        // move body
        for i in (1..self.snake.len()).rev()
        {
            self.snake[i] = self.snake[i - 1];
        }
    }
}