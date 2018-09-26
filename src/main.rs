#![deny(missing_docs)]

//! A Fill-a-pix viewer

extern crate piston_window;

use piston_window::{clear, PistonWindow, WindowSettings};

pub use picgrid::PictureGrid;
pub use picgrid_controller::PictureGridController;
pub use picgrid_view::{PictureGridView, PictureGridViewSettings};

mod picgrid;
mod picgrid_controller;
mod picgrid_view;

fn main() {
    let picgrid = PictureGrid::new();
    let mut picgrid_controller = PictureGridController::new(picgrid);
    let picgrid_view_settings = PictureGridViewSettings::new();
    let picgrid_view = PictureGridView::new(picgrid_view_settings);

    let mut window: PistonWindow = WindowSettings::new("Fill-a-Pix", [640, 40])
        .exit_on_esc(true)
        .build()
        .unwrap();
    while let Some(event) = window.next() {
        picgrid_controller.event(&event);

        window.draw_2d(&event, |context, graphics| {
            clear([1.0; 4], graphics);
            picgrid_view.draw(&picgrid_controller, &context, graphics);
        });
    }
}
