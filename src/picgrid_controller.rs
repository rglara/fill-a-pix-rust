//! PictureGrid controller.

use piston_window::generic_event::GenericEvent;

use PictureGrid;

/// Handles events for Fill-a-Pix grid.
pub struct PictureGridController {
    /// Stores the picture grid state.
    pub picgrid: PictureGrid,
}

impl PictureGridController {
    /// Creates a new picgrid controller.
    pub fn new(picgrid: PictureGrid) -> PictureGridController {
        PictureGridController { picgrid: picgrid }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, _e: &E) {}
}
