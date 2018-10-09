//! PictureGrid view.

use piston_window::types::Color;

/// Stores picgrid view settings.
pub struct PictureGridViewSettings {
    /// background color
    pub background_color: Color,
    /// (x,y) position of upper left corner of controls area
    pub controls_position: [f64; 2],
    /// (x,y) position of upper left corner of grid
    pub grid_position: [f64; 2],
    /// (h,v) side margins within view port
    pub margin: [f64; 2],
    /// size of label text
    pub label_size: u32,
    /// color of control label text
    pub label_color: Color,
    /// width/height of grid cells (only if viewport cannot be determined)
    pub cell_size: f64,
    /// color of grid borders
    pub grid_border_color: Color,
    /// width of outer grid border
    pub grid_border_width: f64,
    /// width of cell grid border
    pub cell_border_width: f64,
    /// color of hint text (when cell is unsolved)
    pub cell_unsolved_hint_text_color: Color,
    /// background color of cell (when cell is unsolved)
    pub cell_unsolved_background_color: Color,
    /// color of hint text (when cell is solved as shaded)
    pub cell_solved_shaded_hint_text_color: Color,
    /// background color of cell (when cell is solved as shaded)
    pub cell_solved_shaded_background_color: Color,
    /// color of hint text (when cell is solved as unshaded)
    pub cell_solved_unshaded_hint_text_color: Color,
    /// background color of cell (when cell is solved as unshaded)
    pub cell_solved_unshaded_background_color: Color,
}

impl PictureGridViewSettings {
    /// Creates new picgrid view settings.
    pub fn new() -> PictureGridViewSettings {
        PictureGridViewSettings {
            background_color: [0.89, 0.87, 0.73, 1.0],
            controls_position: [15.0; 2],
            grid_position: [260.0, 15.0],
            margin: [15.0; 2],
            label_size: 15,
            label_color: [0.0, 0.0, 0.0, 1.0],
            cell_size: 50.0,
            grid_border_color: [0.4, 0.4, 0.4, 1.0],
            grid_border_width: 3.0,
            cell_border_width: 1.0,
            cell_unsolved_hint_text_color: [0.0, 0.0, 0.0, 1.0],
            cell_unsolved_background_color: [1.0; 4],
            cell_solved_shaded_hint_text_color: [1.0; 4],
            cell_solved_shaded_background_color: [0.0, 0.0, 0.0, 1.0],
            cell_solved_unshaded_hint_text_color: [0.5, 0.5, 0.5, 1.0],
            cell_solved_unshaded_background_color: [0.9, 0.9, 0.9, 1.0],
        }
    }
}

/// Tracking information for window details
pub struct PictureGridView {
    /// Stores the last cell position (if any)
    pub cell_pos: Option<[isize; 2]>,
    /// Stores the last cursor position
    pub cursor_pos: [f64; 2],
    /// Calculated cell size
    pub cell_size: f64,
    /// Calculated position/size of grid
    pub grid_rect: [f64; 4],
}

impl PictureGridView {
    /// Creates a new picgrid view.
    pub fn new() -> PictureGridView {
        PictureGridView {
            cell_pos: None,
            cursor_pos: [0.0; 2],
            cell_size: 1.0,
            grid_rect: [1.0; 4],
        }
    }
}
