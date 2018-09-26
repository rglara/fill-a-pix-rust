//! PictureGrid view.

use piston_window::context::Context;
use piston_window::Graphics;

use PictureGridController;

/// Stores picgrid view settings.
pub struct PictureGridViewSettings {}

impl PictureGridViewSettings {
    /// Creates new picgrid view settings.
    pub fn new() -> PictureGridViewSettings {
        PictureGridViewSettings {}
    }
}

/// Stores visual information about a picture grid
pub struct PictureGridView {
    /// Stores picgrid view settings.
    pub settings: PictureGridViewSettings,
}

impl PictureGridView {
    /// Creates a new picgrid view.
    pub fn new(settings: PictureGridViewSettings) -> PictureGridView {
        PictureGridView { settings: settings }
    }

    /// Draw picture grid.
    pub fn draw<G: Graphics>(&self, _controller: &PictureGridController, _c: &Context, _g: &mut G) {
    }
}
