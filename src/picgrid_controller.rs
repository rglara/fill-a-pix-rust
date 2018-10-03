//! PictureGrid controller.

use piston_window::generic_event::GenericEvent;

use PictureGrid;

/// Handles events for Fill-a-Pix grid.
pub struct PictureGridController {
    /// Stores the picture grid state.
    pub picgrid: PictureGrid,
    /// Stores the last cell position (if any)
    pub cell_pos: Option<[usize; 2]>,
    /// Stores the last cursor position
    pub cursor_pos: [f64; 2],
}

impl PictureGridController {
    /// Creates a new picgrid controller.
    pub fn new(picgrid: PictureGrid) -> PictureGridController {
        PictureGridController {
            picgrid: picgrid,
            cell_pos: None,
            cursor_pos: [0.0; 2],
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, grid_rect: [f64; 4], cell_size: f64, e: &E) {
        if let Some(cp) = e.mouse_cursor_args() {
            self.cursor_pos = cp;
        }

        if self.cursor_pos[0] > grid_rect[0]
            && self.cursor_pos[1] > grid_rect[1]
            && self.cursor_pos[0] < (grid_rect[0] + grid_rect[2])
            && self.cursor_pos[1] < (grid_rect[1] + grid_rect[3])
        {
            // we're within the grid, so see what cell we're in
            self.cell_pos = Some([
                ((self.cursor_pos[0] - grid_rect[0]) / (cell_size + 1.0)).trunc() as usize,
                ((self.cursor_pos[1] - grid_rect[1]) / (cell_size + 1.0)).trunc() as usize,
            ]);
        } else {
            self.cell_pos = None;
        }
    }
}
