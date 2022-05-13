extern crate find_folder;
extern crate piston_window;
extern crate rand;
mod draw;
mod game;
mod snake;
use draw::to_coord_u32;
use game::Game;
use piston_window::types::Color;
use piston_window::*;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];
fn main() {
    let (width, height) = (40, 40);
    let mut window: PistonWindow = WindowSettings::new("start scren", [200, 200])
        .exit_on_esc(true)
        //.opengl(OpenGL::V2_1) // Set a different OpenGl version
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let mut glyphs = window
        .load_font(assets.join("FiraSans-Regular.ttf"))
        .unwrap();

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, device| {
            let transform = c.transform.trans(10.0, 100.0);

            clear([0.0, 0.0, 0.0, 1.0], g);
            text::Text::new_color([0.0, 1.0, 0.0, 1.0], 32)
                .draw(
                    "choose size please",
                    &mut glyphs,
                    &c.draw_state,
                    transform,
                    g,
                )
                .unwrap();

            // Update glyphs before rendering.
            glyphs.factory.encoder.flush(device);
        });
    }
    start_game((width, height));
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
