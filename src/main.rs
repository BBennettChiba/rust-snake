extern crate piston_window;
extern crate rand;
mod draw;
mod game;
mod snake;
use draw::to_coord_u32;
use game::Game;
use piston_window::types::Color;
use piston_window::*;
use std::env;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 | 1 | 2 => panic!("Not enough args"),
        _ => (),
    };
    println!("{:?}", args);
    let width = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(e) => panic!("{}", e),
    };
    let height = match args[2].parse::<i32>() {
        Ok(num) => num,
        Err(e) => panic!("{}", e),
    };
    if height < 10 || width < 10 {
        panic!("dimensions are too small")
    }
    start_game((width, height))
}

fn start_game(dimensions: (i32, i32)) {
    let (width, height) = dimensions;
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key)
        };

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
