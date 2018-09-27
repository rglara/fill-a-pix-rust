#![deny(missing_docs)]

//! A Fill-a-pix viewer

extern crate find_folder;
extern crate piston_window;

use piston_window::types::Color;
use piston_window::{clear, Filter, Glyphs, PistonWindow, TextureSettings, WindowSettings};

pub use picgrid::{CellState, PictureGrid};
pub use picgrid_controller::PictureGridController;
pub use picgrid_view::{PictureGridView, PictureGridViewSettings};

mod picgrid;
mod picgrid_controller;
mod picgrid_view;

const BGCOLOR: Color = [0.89, 0.87, 0.73, 1.0];

fn main() {
    let picgrid = PictureGrid::new(5, 5).with_values(vec![
        CellState::Unshaded(0),
        CellState::Unshaded(PictureGrid::EMPTY),
        CellState::Shaded(4),
        CellState::Shaded(4),
        CellState::Unshaded(PictureGrid::EMPTY),
        CellState::Unshaded(PictureGrid::EMPTY),
        CellState::Unshaded(4),
        CellState::Shaded(PictureGrid::EMPTY),
        CellState::Shaded(6),
        CellState::Unshaded(PictureGrid::EMPTY),
        CellState::Unsolved(3),
        CellState::Unsolved(PictureGrid::EMPTY),
        CellState::Unsolved(7),
        CellState::Shaded(6),
        CellState::Shaded(PictureGrid::EMPTY),
        CellState::Unsolved(PictureGrid::EMPTY),
        CellState::Unsolved(6),
        CellState::Unsolved(PictureGrid::EMPTY),
        CellState::Unsolved(6),
        CellState::Unsolved(5),
        CellState::Unsolved(PictureGrid::EMPTY),
        CellState::Unsolved(PictureGrid::EMPTY),
        CellState::Unsolved(PictureGrid::EMPTY),
        CellState::Unsolved(PictureGrid::EMPTY),
        CellState::Unsolved(3),
    ]);
    let mut picgrid_controller = PictureGridController::new(picgrid);
    let picgrid_view_settings = PictureGridViewSettings::new();
    let picgrid_view = PictureGridView::new(picgrid_view_settings);

    let mut window: PistonWindow = WindowSettings::new("Fill-a-Pix", [400, 400])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    let ref font = assets.join("FiraSans-Regular.ttf");
    let factory = window.factory.clone();
    let texture_settings = TextureSettings::new().filter(Filter::Nearest);
    let mut glyphs = Glyphs::new(font, factory, texture_settings).unwrap();

    while let Some(event) = window.next() {
        picgrid_controller.event(&event);

        window.draw_2d(&event, |context, graphics| {
            clear(BGCOLOR, graphics);
            picgrid_view.draw(&picgrid_controller, &mut glyphs, &context, graphics);
        });
    }
}
