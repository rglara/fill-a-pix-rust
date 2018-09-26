#![deny(missing_docs)]

//! A Fill-a-pix viewer

extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Fill-a-Pix", [640, 40])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, |_context, graphics| {
            clear([1.0; 4], graphics);
        });
    }
}
