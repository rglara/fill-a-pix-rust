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

impl CellState {
    /// Returns the common hint value in the enumeration
    pub fn hint(self) -> u8 {
        match self {
            CellState::Unsolved(val) => val,
            CellState::Shaded(val) => val,
            CellState::Unshaded(val) => val,
        }
    }
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
            cells: vec![CellState::Unsolved(PictureGrid::EMPTY); (width * height) as usize],
        }
    }

    /// Sets all cell values
    pub fn with_values(&mut self, values: Vec<CellState>) -> &Self {
        self.cells = values;
        self
    }

    /// Get individual cell value
    pub fn get(&self, x: isize, y: isize) -> CellState {
        self.cells[(y * self.width as isize + x) as usize].clone()
    }

    /// Set individual cell value
    pub fn set(&mut self, x: isize, y: isize, value: CellState) -> &Self {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.cells[(y * self.width as isize + x) as usize] = value;
        }
        self
    }

    /// Set individual cell value state only (value is unchanged)
    pub fn set_state(&mut self, x: isize, y: isize, value: CellState) -> &Self {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = (y * self.width as isize + x) as usize;
            let existing_hint;
            match self.cells[index] {
                CellState::Unsolved(hint) => existing_hint = hint,
                CellState::Shaded(hint) => existing_hint = hint,
                CellState::Unshaded(hint) => existing_hint = hint,
            }
            let new_value;
            match value {
                CellState::Unsolved(_hint) => new_value = CellState::Unsolved(existing_hint),
                CellState::Shaded(_hint) => new_value = CellState::Shaded(existing_hint),
                CellState::Unshaded(_hint) => new_value = CellState::Unshaded(existing_hint),
            }
            self.cells[(y * self.width as isize + x) as usize] = new_value;
        }
        self
    }
}
