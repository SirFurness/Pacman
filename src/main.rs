extern crate piston_window;

use piston_window::*;

const WIDTH: u32 = 512;
const HEIGHT: u32 = 384;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Pacman", [WIDTH, HEIGHT])
        .resizable(false).build().unwrap();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.5, 0.5, 0.5, 1.0], g);
            rectangle([1.0, 0.0, 0.0, 1.0],
                      [0.0, 0.0, 100.0, 100.0],
                      c.transform, g);
        });
    }
}
