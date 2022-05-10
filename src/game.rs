use piston_window::types::Color;
use piston_window::*;

use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};
use rand::thread_rng;
use rand::Rng;

const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const BORDER_COLOR: Color = [0.0, 0.0, 0.0, 1.0];
const GAMEOVER_COLOR: Color = [0.9, 0.0, 0.0, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_x: i32,
    food_y: i32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),

            food_exists: true,
            food_x: 6,
            food_y: 4,

            width,
            height,

            game_over: false,
            waiting_time: 0.0,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::K => Some(Direction::Up),
            Key::J => Some(Direction::Down),
            Key::H => Some(Direction::Left),
            Key::L => Some(Direction::Right),
            _ => None,
        };

        if dir.is_none() {
            return;
        }

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.is_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn spawn_food(&mut self) {
        let mut rng = thread_rng();
        let mut new_x = rng.gen_range(1..self.width - 1);
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }

    pub fn update(&mut self, dt: f64) {
        self.waiting_time += dt;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.spawn_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None)
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d) {
        self.snake.draw(context, graphics);
        if self.food_exists {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, graphics);
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, context, graphics);
        draw_rectangle(
            BORDER_COLOR,
            0,
            self.height - 1,
            self.width,
            1,
            context,
            graphics,
        );
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, context, graphics);
        draw_rectangle(
            BORDER_COLOR,
            self.width - 1,
            0,
            1,
            self.height,
            context,
            graphics,
        );
        if self.game_over {
            draw_rectangle(
                GAMEOVER_COLOR,
                0,
                0,
                self.width,
                self.height,
                context,
                graphics,
            );
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y) = self.snake.head_position();
        if self.food_exists && head_y == self.food_y && head_x == self.food_x {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn is_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }
}
