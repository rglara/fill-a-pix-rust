//! Grid for picture.

/// Width of grid.
const WIDTH: usize = 5;

/// Height of grid.
const HEIGHT: usize = 5;

/// Stores picture grid data.
pub struct PictureGrid {
    /// Stores the content of the grid cells.
    pub cells: [[u8; WIDTH]; HEIGHT],
}

impl PictureGrid {
    /// Signifies that cell has no hint.
    pub const EMPTY: u8 = 10;

    /// Creates new picture grid.
    pub fn new() -> PictureGrid {
        PictureGrid {
            cells: [[PictureGrid::EMPTY; WIDTH]; HEIGHT],
        }
    }
}
