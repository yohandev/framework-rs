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
/// period between food generation, in tick(frames)
const FOOD_RATE: u64 = 40;

struct SnakeGame
{
    /// snake's tile positions, where `snake[0]` is
    /// the head
    snake: Vec<Vec2<i32>>,
    /// snake's direction, either ±[1, 0] or ±[0, 1]
    /// starts at [0, 0] for no direction
    dir: Vec2<i32>,

    /// position of food in map
    food: Vec<Vec2<i32>>,
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

        // snake style
        c.fill(c!("honeydew"));
        c.no_stroke();

        // draw snake
        for tile in &self.snake
        {
            c.rect(tile * TILE_SIZE, TILE_SIZE);
        }

        // food style
        c.fill(c!("darksalmon"));

        // draw food
        for tile in &self.food
        {
            c.rect(tile * TILE_SIZE, TILE_SIZE);
        }
    }

    fn update(&mut self, app: &mut App)
    {
        // time
        app.time().print_current_frame_rate();

        // input
        if app.keys().pressed(btn!("left"))  && self.dir.x !=  1 { self.dir = v![-1, 0]; }
        if app.keys().pressed(btn!("right")) && self.dir.x != -1 { self.dir = v![ 1, 0]; }
        if app.keys().pressed(btn!("up"))    && self.dir.y !=  1 { self.dir = v![ 0,-1]; }
        if app.keys().pressed(btn!("down"))  && self.dir.y != -1 { self.dir = v![ 0, 1]; }

        // no movement
        if self.dir == v![0, 0]
        {   
            return;
        }

        // generate food
        if (app.time().tick() - 1) % FOOD_RATE == 0
        {
            // predicates:
            // 1. food cannot intersect other food
            // 2. food cannot intersect snake
            let pos = loop
            {
                let x = app.random().gen_range(0..GRID_SIZE.x);
                let y = app.random().gen_range(0..GRID_SIZE.y);
                
                let pos = v![x, y];

                if !self.snake.iter().any(|&t| t == pos)
                && !self.food.iter().any(|&f| f == pos)
                {
                    break pos;
                }
            };
            self.food.push(pos);
        }

        // move head
        self.snake[0] += self.dir;
        
        // out of grid -> game over!
        let neg_grid = self.snake[0]
            .is_any_negative();
        let pos_grid = self.snake[0]
            .cmpge(&GRID_SIZE)
            .reduce_or();
        // head intersects with body -> game over!
        let intersect = self.snake
            .iter()
            .skip(1)
            .any(|&t| t == self.snake[0]);

        // test game over
        if neg_grid || pos_grid || intersect
        {
            println!("GAME OVER!\nscore: {}", self.snake.len());

            return *self = Self::default();
        }

        // find food at head
        let food = self.food
            .iter()
            .enumerate()
            .find(|(_, &t)| t == self.snake[0]);

        // eat food
        if let Some((ind, _)) = food
        {
            // grow snake
            self.snake.push(*self.snake.last().unwrap());

            // consume food
            self.food.remove(ind);
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
            food: vec![]
        }
    }
}