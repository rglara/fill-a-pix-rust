//! PictureGrid controller.

use piston_window::generic_event::GenericEvent;
use piston_window::{Button, Key, MouseButton};

use picgrid::CellState;

use PictureGrid;

/// Handles events for Fill-a-Pix grid.
pub struct PictureGridController {
    /// Stores the picture grid state.
    pub picgrid: PictureGrid,
    /// Stores the last cell position (if any)
    pub cell_pos: Option<[isize; 2]>,
    /// Stores the last cursor position
    pub cursor_pos: [f64; 2],
    /// Stores if the solving algorithm is active
    is_solving: bool,
    /// Stores if solving algorithm needs another pass
    algorithm_needs_pass: bool,
    /// Determines how many steps to perform per update event
    steps_per_update: u16,
}

impl PictureGridController {
    /// Creates a new picgrid controller.
    pub fn new(picgrid: PictureGrid) -> PictureGridController {
        let initial_steps = picgrid.width * 2;
        PictureGridController {
            picgrid: picgrid,
            cell_pos: None,
            cursor_pos: [0.0; 2],
            is_solving: false,
            algorithm_needs_pass: false,
            steps_per_update: initial_steps,
        }
    }

    /// Returns messages to display
    pub fn get_messages(&self) -> Vec<String> {
        let mut ret_val = vec![
            "Press 'x' to toggle algorithm".to_string(),
            format!("Steps per Update: {} ('+'/'-')", self.steps_per_update),
        ];
        if let Some(pos) = self.cell_pos {
            ret_val.push(format!("Processing ({},{})", pos[0], pos[1]));
        }
        ret_val
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, grid_rect: [f64; 4], cell_size: f64, e: &E) {
        if !self.is_solving {
            if let Some(cp) = e.mouse_cursor_args() {
                self.cursor_pos = cp;
                if self.cursor_pos[0] > grid_rect[0]
                    && self.cursor_pos[1] > grid_rect[1]
                    && self.cursor_pos[0] < (grid_rect[0] + grid_rect[2])
                    && self.cursor_pos[1] < (grid_rect[1] + grid_rect[3])
                {
                    // we're within the grid, so see what cell we're in (size + border width)
                    self.cell_pos = Some([
                        ((self.cursor_pos[0] - grid_rect[0]) / (cell_size + 1.0)).trunc() as isize,
                        ((self.cursor_pos[1] - grid_rect[1]) / (cell_size + 1.0)).trunc() as isize,
                    ]);
                } else {
                    self.cell_pos = None;
                }
            }

            if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
                if let Some(pos) = self.cell_pos {
                    if let Some(cell) = self.picgrid.get(pos[0], pos[1]) {
                        let new_state;
                        match cell {
                            CellState::Unsolved(val) => new_state = CellState::Shaded(val),
                            CellState::Shaded(val) => new_state = CellState::Unshaded(val),
                            CellState::Unshaded(val) => new_state = CellState::Unsolved(val),
                        };
                        self.picgrid.set(pos[0], pos[1], new_state);
                    }
                }
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::X => {
                    self.is_solving = !self.is_solving;
                    if self.is_solving {
                        self.cell_pos = Some([0, 0]);
                    } else {
                        self.cell_pos = None;
                    }
                }
                Key::NumPadPlus => {
                    self.steps_per_update += 1;
                }
                Key::NumPadMinus => {
                    self.steps_per_update -= 1;
                    if self.steps_per_update < 1 {
                        self.steps_per_update = 1;
                    }
                }
                _ => {}
            }
        }

        if let Some(_) = e.update_args() {
            for _loop in 0..self.steps_per_update {
                if self.is_solving {
                    if let Some(cell) = self.cell_pos {
                        let mut x = cell[0];
                        let mut y = cell[1];
                        let (nx, ny, ncell) = self.picgrid.next_incomplete(x, y);
                        if let Some(cell) = ncell {
                            x = nx;
                            y = ny;
                            let cell_hint = cell.hint();
                            let cell_shaded = self.picgrid.num_shaded(x, y);
                            let cell_unsolved = self.picgrid.num_unsolved(x, y);
                            if cell_hint == cell_shaded {
                                self.picgrid.fill_unshaded(x, y);
                                self.algorithm_needs_pass = true;
                            } else if cell_hint == (cell_shaded + cell_unsolved) {
                                self.picgrid.fill_shaded(x, y);
                                self.algorithm_needs_pass = true;
                            }
                        } else {
                            x = -1;
                            y = 0;
                        }

                        x += 1;
                        if x >= self.picgrid.width as isize {
                            x = 0;
                            y += 1;
                        }
                        if y < self.picgrid.height as isize {
                            self.cell_pos = Some([x, y]);
                        } else if self.algorithm_needs_pass {
                            self.cell_pos = Some([0, 0]);
                            self.algorithm_needs_pass = false;
                        } else {
                            self.cell_pos = None;
                            self.is_solving = false;
                            self.algorithm_needs_pass = false;
                        }
                    }
                } else {
                    self.cell_pos = None;
                }
            }
        }
    }
}
