//! Grid for picture.

/// Stores picture grid data.
pub struct PictureGrid {
    /// Width of grid.
    pub width: u16,
    /// Height of grid.
    pub height: u16,
    /// Stores the content of the grid cells.
    pub cells: Vec<u8>,
}

impl PictureGrid {
    /// Signifies that cell has no hint.
    pub const EMPTY: u8 = 10;

    /// Creates new picture grid.
    pub fn new(width: u16, height: u16) -> PictureGrid {
        PictureGrid {
            width: width,
            height: height,
            cells: vec![PictureGrid::EMPTY; usize::from(width * height)],
        }
    }
}
