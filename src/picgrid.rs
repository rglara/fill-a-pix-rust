//! Grid for picture.

use std::collections::HashMap;

/// Enumeration of cell states
#[derive(Copy, Clone, Serialize, Deserialize)]
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
    pub cells: HashMap<isize, CellState>,
}

impl PictureGrid {
    /// Signifies that cell has no hint.
    pub const EMPTY: u8 = 10;

    /// Creates new picture grid.
    pub fn new(width: u16, height: u16) -> PictureGrid {
        PictureGrid {
            width: width,
            height: height,
            cells: HashMap::with_capacity((width * height) as usize),
            //vec![CellState::Unsolved(PictureGrid::EMPTY); (width * height) as usize],
        }
    }

    /// Sets all cell values
    pub fn with_values(&mut self, values: HashMap<isize, CellState>) -> &Self {
        self.cells = values;
        self
    }

    /// Get individual cell value
    pub fn get(&self, x: isize, y: isize) -> Option<CellState> {
        let mut ret_val = Some(CellState::Unshaded(PictureGrid::EMPTY));
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            ret_val = Some(self.cells[&(y * self.width as isize + x)].clone());
        }
        ret_val
    }

    /// Get next incomplete cell, starting at (x,y) (without wrapping around)
    pub fn next_incomplete(&self, x: isize, y: isize) -> (isize, isize, Option<CellState>) {
        let mut next_x = x;
        let mut next_y = y;
        while (next_x < (self.width as isize)) && (next_y < (self.height as isize)) {
            if let Some(cell) = self.get(next_x, next_y) {
                let cell_hint = cell.hint();
                if cell_hint != PictureGrid::EMPTY && !self.is_complete(next_x, next_y) {
                    return (next_x, next_y, Some(cell));
                }
            }
            next_x += 1;
            if next_x >= self.width as isize {
                next_x = 0;
                next_y += 1;
            }
        }
        (x, y, None)
    }

    /// Set individual cell value
    pub fn set(&mut self, x: isize, y: isize, value: CellState) -> &Self {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            self.cells.insert(y * self.width as isize + x, value);
        }
        self
    }

    /// Set individual cell value state only (value is unchanged)
    fn internal_set_state(
        &mut self,
        x: isize,
        y: isize,
        value: CellState,
        unsolved_only: bool,
    ) -> &Self {
        if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
            let index = y * self.width as isize + x;
            let mut existing_hint: Option<u8> = None;
            match self.cells[&index] {
                CellState::Unsolved(hint) => existing_hint = Some(hint),
                CellState::Shaded(hint) => {
                    if !unsolved_only {
                        existing_hint = Some(hint);
                    }
                }
                CellState::Unshaded(hint) => {
                    if !unsolved_only {
                        existing_hint = Some(hint);
                    }
                }
            }
            if let Some(hint) = existing_hint {
                let new_value;
                match value {
                    CellState::Unsolved(_hint) => new_value = CellState::Unsolved(hint),
                    CellState::Shaded(_hint) => new_value = CellState::Shaded(hint),
                    CellState::Unshaded(_hint) => new_value = CellState::Unshaded(hint),
                }
                self.cells.insert(index, new_value);
            }
        }
        self
    }

    /// Set individual cell value state only (value is unchanged)
    pub fn set_state(&mut self, x: isize, y: isize, value: CellState) -> &Self {
        self.internal_set_state(x, y, value, false)
    }

    /// Set individual cell value state only (value is unchanged) for unsolved cells only
    pub fn set_unsolved_state(&mut self, x: isize, y: isize, value: CellState) -> &Self {
        self.internal_set_state(x, y, value, true)
    }

    /// Fills surrounding grid with shaded cells
    pub fn fill_shaded(&mut self, x: isize, y: isize) -> &Self {
        self.set_unsolved_state(x - 1, y - 1, CellState::Shaded(0));
        self.set_unsolved_state(x, y - 1, CellState::Shaded(0));
        self.set_unsolved_state(x + 1, y - 1, CellState::Shaded(0));
        self.set_unsolved_state(x - 1, y, CellState::Shaded(0));
        self.set_unsolved_state(x, y, CellState::Shaded(0));
        self.set_unsolved_state(x + 1, y, CellState::Shaded(0));
        self.set_unsolved_state(x - 1, y + 1, CellState::Shaded(0));
        self.set_unsolved_state(x, y + 1, CellState::Shaded(0));
        self.set_unsolved_state(x + 1, y + 1, CellState::Shaded(0));

        self
    }

    /// Fills surrounding grid with unshaded cells
    pub fn fill_unshaded(&mut self, x: isize, y: isize) -> &Self {
        self.set_unsolved_state(x - 1, y - 1, CellState::Unshaded(0));
        self.set_unsolved_state(x, y - 1, CellState::Unshaded(0));
        self.set_unsolved_state(x + 1, y - 1, CellState::Unshaded(0));
        self.set_unsolved_state(x - 1, y, CellState::Unshaded(0));
        self.set_unsolved_state(x, y, CellState::Unshaded(0));
        self.set_unsolved_state(x + 1, y, CellState::Unshaded(0));
        self.set_unsolved_state(x - 1, y + 1, CellState::Unshaded(0));
        self.set_unsolved_state(x, y + 1, CellState::Unshaded(0));
        self.set_unsolved_state(x + 1, y + 1, CellState::Unshaded(0));

        self
    }

    /// Finds number of known shaded cells in surrounding grid
    pub fn num_shaded(&self, x: isize, y: isize) -> u8 {
        let mut num = 0;
        for a in (x - 1)..(x + 2) {
            for b in (y - 1)..(y + 2) {
                if let Some(cell) = self.get(a, b) {
                    match cell {
                        CellState::Shaded(_hint) => num += 1,
                        _ => {}
                    }
                }
            }
        }
        num
    }

    /// Finds number of known unshaded cells in surrounding grid
    pub fn num_unshaded(&self, x: isize, y: isize) -> u8 {
        let mut num = 0;
        for a in (x - 1)..(x + 2) {
            for b in (y - 1)..(y + 2) {
                if let Some(cell) = self.get(a, b) {
                    match cell {
                        CellState::Unshaded(_hint) => num += 1,
                        _ => {}
                    }
                }
            }
        }
        num
    }

    /// Finds number of unsolved cells in surrounding grid
    pub fn num_unsolved(&self, x: isize, y: isize) -> u8 {
        let mut num = 0;
        for a in (x - 1)..(x + 2) {
            for b in (y - 1)..(y + 2) {
                if let Some(cell) = self.get(a, b) {
                    match cell {
                        CellState::Unsolved(_hint) => num += 1,
                        _ => {}
                    }
                }
            }
        }
        num
    }

    /// Finds if surrounding grid is complete (no unsolved)
    pub fn is_complete(&self, x: isize, y: isize) -> bool {
        let mut num_complete = 0;
        for a in (x - 1)..(x + 2) {
            for b in (y - 1)..(y + 2) {
                if let Some(cell) = self.get(a, b) {
                    match cell {
                        CellState::Unshaded(_hint) => num_complete += 1,
                        CellState::Shaded(_hint) => num_complete += 1,
                        _ => {}
                    }
                }
            }
        }
        num_complete == 9
    }
}
