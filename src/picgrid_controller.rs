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
                let new_state;
                match self.picgrid.get(pos[0], pos[1]) {
                    CellState::Unsolved(val) => new_state = CellState::Shaded(val),
                    CellState::Shaded(val) => new_state = CellState::Unshaded(val),
                    CellState::Unshaded(val) => new_state = CellState::Unsolved(val),
                };
                self.picgrid.set(pos[0], pos[1], new_state);
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::X => self.run_solver(),
                _ => {}
            }
        }
    }

    /// Executes solving algorithm
    fn run_solver(&mut self) {
        println!("Solving picture grid...");
        let mut x: isize = 0;
        let mut y: isize = 0;
        for _index in 0..(self.picgrid.width * self.picgrid.height) {
            println!("Processing ({},{})", x, y);
            let cell = self.picgrid.get(x, y);
            match cell {
                CellState::Unsolved(val) => match val {
                    0 => {
                        self.picgrid
                            .set_state(x - 1, y - 1, CellState::Unshaded(val));
                        self.picgrid.set_state(x, y - 1, CellState::Unshaded(val));
                        self.picgrid
                            .set_state(x + 1, y - 1, CellState::Unshaded(val));
                        self.picgrid.set_state(x - 1, y, CellState::Unshaded(val));
                        self.picgrid.set_state(x, y, CellState::Unshaded(val));
                        self.picgrid.set_state(x + 1, y, CellState::Unshaded(val));
                        self.picgrid
                            .set_state(x - 1, y + 1, CellState::Unshaded(val));
                        self.picgrid.set_state(x, y + 1, CellState::Unshaded(val));
                        self.picgrid
                            .set_state(x + 1, y + 1, CellState::Unshaded(val));
                    }
                    9 => {
                        self.picgrid.set_state(x - 1, y - 1, CellState::Shaded(val));
                        self.picgrid.set_state(x, y - 1, CellState::Shaded(val));
                        self.picgrid.set_state(x + 1, y - 1, CellState::Shaded(val));
                        self.picgrid.set_state(x - 1, y, CellState::Shaded(val));
                        self.picgrid.set_state(x, y, CellState::Shaded(val));
                        self.picgrid.set_state(x + 1, y, CellState::Shaded(val));
                        self.picgrid.set_state(x - 1, y + 1, CellState::Shaded(val));
                        self.picgrid.set_state(x, y + 1, CellState::Shaded(val));
                        self.picgrid.set_state(x + 1, y + 1, CellState::Shaded(val));
                    }
                    _ => {}
                },
                _ => {}
            }
            x += 1;
            if x > self.picgrid.width as isize {
                x = 0;
                y += 1;
            }
        }
        println!("Done with algorithm!");
    }
}
