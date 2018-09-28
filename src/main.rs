#![deny(missing_docs)]

//! A Fill-a-pix viewer

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

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
    let picgrid: PictureGrid;
    if let Some(filename) = std::env::args().nth(1) {
        println!("Loading {}...", filename);

        // TODO: load the real file
        let loaded_file = "{\"width\":5,\"height\":5,\"cells\":[{\"Unsolved\":0},{\"Unsolved\":10},{\"Unsolved\":4},{\"Unsolved\":4},{\"Unsolved\":10},{\"Unsolved\":10},{\"Unsolved\":4},{\"Unsolved\":10},{\"Unsolved\":6},{\"Unsolved\":10},{\"Unsolved\":3},{\"Unsolved\":10},{\"Unsolved\":7},{\"Unsolved\":6},{\"Unsolved\":10},{\"Unsolved\":10},{\"Unsolved\":6},{\"Unsolved\":10},{\"Unsolved\":6},{\"Unsolved\":5},{\"Unsolved\":10},{\"Unsolved\":10},{\"Unsolved\":10},{\"Unsolved\":10},{\"Unsolved\":3}]}";

        let deserialized_try = serde_json::from_str(&loaded_file);
        if deserialized_try.is_ok() {
            picgrid = deserialized_try.unwrap();
            println!("{} loaded!", filename)
        } else {
            println!("Unable to load {}", filename);
            std::process::exit(2);
        }
    } else {
        println!("No filename provided");
        std::process::exit(1);
    }

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
