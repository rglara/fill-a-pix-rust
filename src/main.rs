#![deny(missing_docs)]

//! A Fill-a-pix viewer

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate find_folder;
extern crate piston_window;

use std::fs::File;
use std::io;

use serde_json::error;

use piston_window::{PistonWindow, WindowSettings};

pub use picgrid::PictureGrid;
pub use picgrid_controller::PictureGridController;

mod picgrid;
mod picgrid_controller;
mod picgrid_view;

const DEFAULT_WINDOW: [u32; 2] = [1440, 900];

fn load_file(filename: &String) -> Result<PictureGrid, i32> {
    println!("Loading {}...", filename);

    let io_err = |err: io::Error| -> i32 {
        if let Some(raw_os_err) = err.raw_os_error() {
            raw_os_err
        } else {
            1
        }
    };
    let input = File::open(filename).map_err(io_err)?;

    let serde_err = |err: error::Error| -> i32 {
        match err.classify() {
            error::Category::Io => 1001,
            error::Category::Syntax => 1002,
            error::Category::Data => 1003,
            error::Category::Eof => 1004,
        }
    };
    let picgrid = serde_json::from_reader(input).map_err(serde_err)?;
    println!("{} loaded!", filename);
    Ok(picgrid)
}

fn main() {
    let picgrid: PictureGrid;
    if let Some(filename) = std::env::args().nth(1) {
        picgrid = match load_file(&filename) {
            Ok(pg) => pg,
            Err(error_code) => {
                println!("Error({}): Unable to load {}", error_code, filename);
                std::process::exit(error_code);
            }
        };
    } else {
        let error_code = 1;
        println!("Error({}): No filename provided", error_code);
        std::process::exit(error_code);
    }

    let window: PistonWindow = WindowSettings::new("Fill-a-Pix", DEFAULT_WINDOW)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut picgrid_controller = PictureGridController::new(window, picgrid);
    picgrid_controller.event_pump();
}
