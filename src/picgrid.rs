//! Grid for picture.

/// Enumeration of cell states
#[derive(Clone, Serialize, Deserialize)]
pub enum CellState {
    /// Cell is not solved yet
    Unsolved(u8),
    /// Cell is solved and should be shaded
    Shaded(u8),
    /// Cell is solved and should be unshaded
    Unshaded(u8),
}

/// Stores picture grid data.
#[derive(Serialize, Deserialize)]
pub struct PictureGrid {
    /// Width of grid.
    pub width: u16,
    /// Height of grid.
    pub height: u16,
    /// Stores the content of the grid cells.
    pub cells: Vec<CellState>,
}

impl PictureGrid {
    /// Signifies that cell has no hint.
    pub const EMPTY: u8 = 10;

    /// Creates new picture grid.
    pub fn new(width: u16, height: u16) -> PictureGrid {
        PictureGrid {
            width: width,
            height: height,
            cells: vec![CellState::Unsolved(PictureGrid::EMPTY); usize::from(width * height)],
        }
    }

    /// Sets cell values
    pub fn with_values(mut self, values: Vec<CellState>) -> Self {
        self.cells = values;
        self
    }
}
