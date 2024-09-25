extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use std::os::windows;
use std::process::exit;

use glutin_window::GlutinWindow as Window;
use graphics::{grid, math};
use graphics::types::Width;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use piston::{EventLoop, Key};
use rand::Rng;

pub struct Game{
    gl: GlGraphics,
    snake: Snake, 
    // obsatcle: Obstacle,
    food: Food, 
    width: i32, 
    height: i32, 
}
impl Game {
    fn new(opengl:OpenGL, width:i32, height:i32) -> Game{
        Game{
        // obsatcle: Obstacle,
        gl: GlGraphics::new(opengl),
        snake: Snake::new(width/2, height/2),
        food: Food::spawn_food(width, height),
        width,
        height,
    }
    }
    fn update(&mut self){
        self.snake.update();
        if self.snake.eat(&self.food){
            println!("Food is at  {} {}", self.food.x, self.food.y);
            self.food = Food::spawn_food(self.width, self.height);

        }
        else{
            self.snake.body.pop();
        }
        //Current Location of Snake
        if let Some(&(head_x, head_y)) = self.snake.body.first(){
            // println!("Snake head is at: ({}, {})", head_x, head_y);
            // println!("Grid size is: width = {}, height = {}", self.width, self.height);
        }
        // Cahnging snakes direction when it hits az
        let mut new_head = (*self.snake.body.first().expect("The snake has no body"));
        match self.snake.direction{
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        }
        if new_head.0 == -2{
            Direction::Down;
            
        }
        else if new_head.0 == self.width + 1 {
            panic!("You lose")
        }
        else if new_head.1 == -2{
            panic!("You lose")
        }
        else if new_head.1 == self.height + 1 {
           panic!("You lose")
        }
        
    }
    fn render(&mut self, args:&RenderArgs){
        use graphics;
        let food_colour: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let snake_colour: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        let square_size = 20.0;
        let square_size_food = 10.0;
        self.gl.draw(args.viewport(), |c, gl|{
            graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
            let food_square = graphics::rectangle::square(self.food.x as f64 * square_size ,self.food.y as f64 * square_size, square_size);
            graphics::rectangle(food_colour, food_square, c.transform, gl);

         for block in &self.snake.body{
            let snake_square = graphics::rectangle::square(block.0 as f64 * square_size, block.1 as f64 * square_size, square_size);
            graphics::rectangle(snake_colour, snake_square, c.transform, gl);
         }
        });
    }
    fn key_pressed(&mut self, key: Button){
        self.snake.key_pressed(key);
    }
}
#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// pub struct Obstacle{
//     x: i32,
//     y: i32, 
// }
// impl Obstacle{
//     fn new_obstacle( width:i32, ) -> Obstacle {
//         let mut rng = rand::thread_rng();
        
//     }
// }

pub struct Food{
    x: i32, 
    y: i32,
}
impl Food {
    fn spawn_food(width:i32, height:i32) -> Food{
        let mut rng = rand::thread_rng();
        let grid_cell_size = 20;
        
        let grid_width = width/grid_cell_size;
        let grid_height = height/grid_cell_size;
        let x = rng.gen_range(0..grid_width * 20);
        let y = rng.gen_range(0..grid_height * 20) ;
        Food{
            x, 
            y,
        }
    }
}
pub struct Snake {
    body: Vec<(i32, i32)>,
    direction: Direction,
}
impl Snake {
    fn new(x: i32, y: i32) -> Snake {
        Snake {
            body: vec![(x, y)],
            direction: Direction::Right,
        }
    }
    fn update(&mut self, ){
        let mut new_head = (*self.body.first().expect("The snake has no body"));
        match self.direction{
            Direction::Up => new_head.1 -= 1,
            Direction::Down => new_head.1 += 1,
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
        }
        // if new_head.0 == -1{
        //     self.direction = Direction::Down;
        // }
        // else if new_head.0 == width {
        //     self.direction = Direction::Up;
        // }
        // else if new_head.1 == -1{
        //     self.direction = Direction::Left;
        // }
        // else if new_head.1 == height {
        //     self.direction = Direction::Right;
        // }
        // if new_head.0 == 0{

        // }
        self.body.insert(0, new_head);
        
    }

    fn key_pressed(&mut self, key: Button){
        match key {
            Button::Keyboard(Key::Up) if self.direction != Direction::Down => self.direction = Direction::Up,
            Button::Keyboard(Key::Down) if self.direction != Direction::Up => self.direction = Direction::Down,
            Button::Keyboard(Key::Left) if self.direction != Direction::Right => self.direction = Direction::Left,
            Button::Keyboard(Key::Right) if self.direction != Direction::Left => self.direction = Direction::Right,
            _ => (), 
        }
    }
    fn eat(&mut self, food: &Food) -> bool {
        // Get the position of the snake's head (the first element in the body vector)
        if let Some(&(head_x, head_y)) = self.body.first() {
            // Check if the head's position matches the food's position
            if head_x == food.x && head_y == food.y {
                return true;  // The snake ate the food
            }
        }
        false  // The snake has not eaten the food
    }

}

fn main() {
    let opengl = OpenGL::V4_5;

    let mut windows:Window = WindowSettings::new("Snake Game", [800, 600])
    .exit_on_esc(true)
    .build()
    .unwrap();

    let mut game = Game::new(opengl, 40, 30);
    let mut events = Events::new(EventSettings::new()).ups(8); 


    while let Some(e) = events.next(&mut windows) {
        if let Some(r) = e.render_args() {
            game.render(&r);   // Render the game (draw everything)
        }

        if let Some(u) = e.update_args() {
            game.update();     // Update the game logic (move the snake, check collisions)
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(Button::Keyboard(key));  // Handle key press events
        }
    }
}
