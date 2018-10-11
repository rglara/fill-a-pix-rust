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
    /// Stores on-screen messages
    pub messages: Vec<String>,
    /// Stores if the solving algorithm is active
    is_solving: bool,
    /// Stores if solving algorithm needs another pass
    algorithm_needs_pass: bool,
}

impl PictureGridController {
    /// Creates a new picgrid controller.
    pub fn new(picgrid: PictureGrid) -> PictureGridController {
        PictureGridController {
            picgrid: picgrid,
            cell_pos: None,
            cursor_pos: [0.0; 2],
            messages: vec!["Press 'x' to toggle algorithm".to_string()],
            is_solving: false,
            algorithm_needs_pass: false,
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

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::X => {
                    self.is_solving = !self.is_solving;
                    if self.is_solving {
                        self.cell_pos = Some([0, 0]);
                    }
                }
                _ => {}
            }
        }

        if let Some(_) = e.update_args() {
            if self.is_solving {
                if let Some(cell) = self.cell_pos {
                    let mut x = cell[0] + 1;
                    let mut y = cell[1];

                    self.messages.insert(1, format!("Processing ({},{})", x, y));
                    if self.messages.len() > 2 {
                        self.messages.pop();
                    }

                    if let Some(cell) = self.picgrid.get(x, y) {
                        let cell_hint = cell.hint();
                        if cell_hint != PictureGrid::EMPTY && !self.picgrid.is_complete(x, y) {
                            let cell_shaded = self.picgrid.num_shaded(x, y);
                            let cell_unsolved = self.picgrid.num_unsolved(x, y);
                            if cell_hint == cell_shaded {
                                self.picgrid.fill_unshaded(x, y);
                                self.algorithm_needs_pass = true;
                            } else if cell_hint == (cell_shaded + cell_unsolved) {
                                self.picgrid.fill_shaded(x, y);
                                self.algorithm_needs_pass = true;
                            }
                        }
                    }

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
            }
        }
    }

    // /// Executes solving algorithm
    // fn run_solver(&mut self) {
    //     println!("Solving picture grid...");
    //     let mut x: isize = 0;
    //     let mut y: isize = 0;

    //     println!("Applying zeroes and nines...");
    //     for _index in 0..(self.picgrid.width * self.picgrid.height) {
    //         if let Some(cell) = self.picgrid.get(x, y) {
    //             let cell_hint = cell.hint();
    //             match cell_hint {
    //                 0 => {
    //                     self.picgrid.fill_unshaded(x, y);
    //                 }
    //                 9 => {
    //                     self.picgrid.fill_shaded(x, y);
    //                 }
    //                 6 => {
    //                     if x == 0
    //                         || x == (self.picgrid.width - 1) as isize
    //                         || y == 0
    //                         || y == (self.picgrid.height - 1) as isize
    //                     {
    //                         self.picgrid.fill_shaded(x, y);
    //                     }
    //                 }
    //                 4 => {
    //                     if (x == 0 && y == 0)
    //                         || (x == 0 && y == (self.picgrid.height - 1) as isize)
    //                         || (x == (self.picgrid.width - 1) as isize && y == 0)
    //                         || (x == (self.picgrid.width - 1) as isize
    //                             && y == (self.picgrid.height - 1) as isize)
    //                     {
    //                         self.picgrid.fill_shaded(x, y);
    //                     }
    //                 }
    //                 _ => {}
    //             }
    //             x += 1;
    //             if x >= self.picgrid.width as isize {
    //                 x = 0;
    //                 y += 1;
    //             }
    //         }
    //     }

    //     let mut num_passes = 1;
    //     let mut needs_pass = true;
    //     while needs_pass {
    //         needs_pass = false;
    //         println!("Easy pass #{}", num_passes);

    //         x = 0;
    //         y = 0;
    //         // check if the cell is "satisfied"
    //         for _index in 0..(self.picgrid.width * self.picgrid.height) {
    //             if let Some(cell) = self.picgrid.get(x, y) {
    //                 let cell_hint = cell.hint();
    //                 if cell_hint != PictureGrid::EMPTY && !self.picgrid.is_complete(x, y) {
    //                     let cell_shaded = self.picgrid.num_shaded(x, y);
    //                     let cell_unsolved = self.picgrid.num_unsolved(x, y);
    //                     if cell_hint == cell_shaded {
    //                         self.picgrid.fill_unshaded(x, y);
    //                         needs_pass = true;
    //                     } else if cell_hint == (cell_shaded + cell_unsolved) {
    //                         self.picgrid.fill_shaded(x, y);
    //                         needs_pass = true;
    //                     }
    //                 }
    //             }
    //             x += 1;
    //             if x >= self.picgrid.width as isize {
    //                 x = 0;
    //                 y += 1;
    //             }
    //         }
    //         num_passes += 1;
    //     }

    //     println!("Done with algorithm!");
    // }
}
