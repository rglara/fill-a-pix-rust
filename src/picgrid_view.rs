//! PictureGrid view.

use piston_window::context::Context;
use piston_window::types::Color;
use piston_window::{Graphics, Rectangle};

use PictureGridController;

/// Stores picgrid view settings.
pub struct PictureGridViewSettings {
    /// (x,y) position of upper left corner of grid
    pub position: [f64; 2],
    /// width/height of grid cells
    pub cell_size: f64,
    /// color of grid borders
    pub grid_border_color: Color,
    /// width of outer grid border
    pub grid_border_width: f32,
    /// width of cell grid border
    pub cell_border_width: f32,
    /// size of hint text
    pub cell_hint_text_size: f32,
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
            position: [10.0; 2],
            cell_size: 50.0,
            grid_border_color: [0.2, 0.2, 0.2, 1.0],
            grid_border_width: 2.0,
            cell_border_width: 1.0,
            cell_hint_text_size: 12.0,
            cell_unsolved_hint_text_color: [0.0, 0.0, 0.0, 1.0],
            cell_unsolved_background_color: [1.0; 4],
            cell_solved_shaded_hint_text_color: [1.0; 4],
            cell_solved_shaded_background_color: [0.0, 0.0, 0.0, 1.0],
            cell_solved_unshaded_hint_text_color: [0.2, 0.2, 0.2, 1.0],
            cell_solved_unshaded_background_color: [0.1, 0.1, 0.1, 1.0],
        }
    }
}

/// Stores visual information about a picture grid
pub struct PictureGridView {
    /// Stores picgrid view settings.
    pub settings: PictureGridViewSettings,
}

impl PictureGridView {
    /// Creates a new picgrid view.
    pub fn new(settings: PictureGridViewSettings) -> PictureGridView {
        PictureGridView { settings: settings }
    }

    /// Draw picture grid.
    pub fn draw<G: Graphics>(&self, controller: &PictureGridController, c: &Context, g: &mut G) {
        // use graphics::{Line, Rectangle};

        let ref settings = self.settings;
        let grid_rect = [
            settings.position[0],
            settings.position[1],
            settings.cell_size * f64::from(controller.picgrid.width),
            settings.cell_size * f64::from(controller.picgrid.height),
        ];

        // outer grid border
        Rectangle::new_border(
            settings.grid_border_color,
            (settings.grid_border_width / 2.0).into(),
        ).draw(grid_rect, &c.draw_state, c.transform, g);
    }
}
