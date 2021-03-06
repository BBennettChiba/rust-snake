extern crate piston_window;
extern crate rand;
mod draw;
mod game;
mod snake;
use draw::{draw_rectangle, to_coord_u32};
use piston_window::types::Color;
use piston_window::*;

use game::Game;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
fn main() {
    let (width, height) = (25, 25);
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    let mut game = Game::new(width, height);
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            //game.key_pressed(key)
        };

        window.draw_2d(&event, |c, g, _| {
            clear(BACK_COLOR, g);
            //game.draw(&c, g);
            let mut draw_state = DrawState::new_alpha();
            let text = Text::new(20);
            text.draw("Hello, world!", &mut c, &c.draw_state, c.transform, g);
        });

        event.update(|arg| game.update(arg.dt));
    }
}
