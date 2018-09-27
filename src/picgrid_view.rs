//! PictureGrid view.

use piston_window::context::Context;
use piston_window::types::Color;
use piston_window::Graphics;

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
    pub grid_border_width: f64,
    /// width of cell grid border
    pub cell_border_width: f64,
    /// size of hint text
    pub cell_hint_text_size: u8,
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
            position: [30.0; 2],
            cell_size: 50.0,
            grid_border_color: [0.7, 0.7, 0.7, 1.0],
            grid_border_width: 3.0,
            cell_border_width: 1.0,
            cell_hint_text_size: 12,
            cell_unsolved_hint_text_color: [0.0, 0.0, 0.0, 1.0],
            cell_unsolved_background_color: [1.0; 4],
            cell_solved_shaded_hint_text_color: [1.0; 4],
            cell_solved_shaded_background_color: [0.0, 0.0, 0.0, 1.0],
            cell_solved_unshaded_hint_text_color: [0.7, 0.7, 0.7, 1.0],
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
        use piston_window::rectangle::{Border, Rectangle};

        let ref settings = self.settings;
        let grid_rect = [
            settings.position[0],
            settings.position[1],
            (settings.cell_size * f64::from(controller.picgrid.width)) + settings.grid_border_width,
            (settings.cell_size * f64::from(controller.picgrid.height))
                + settings.grid_border_width,
        ];

        // outer grid border
        Rectangle::new_border(
            settings.grid_border_color,
            (settings.grid_border_width / 2.0).into(),
        ).draw(grid_rect, &c.draw_state, c.transform, g);

        // grid cells
        let cell_unsolved =
            Rectangle::new(settings.cell_unsolved_background_color).border(Border {
                color: settings.grid_border_color,
                radius: (settings.cell_border_width / 2.0).into(),
            });
        let mut column_ptr: u16 = 0;
        let mut row_ptr: u16 = 0;
        let mut cell_rect = [0.0, 0.0, settings.cell_size, settings.cell_size];
        let grid_origin = [
            grid_rect[0] + f64::from(settings.grid_border_width / 2.0),
            grid_rect[1] + f64::from(settings.grid_border_width / 2.0),
        ];
        for _val in &controller.picgrid.cells {
            cell_rect[0] = grid_origin[0] + (f64::from(column_ptr) * settings.cell_size);
            cell_rect[1] = grid_origin[1] + (f64::from(row_ptr) * settings.cell_size);
            cell_unsolved.draw(cell_rect, &c.draw_state, c.transform, g);

            column_ptr += 1;
            if column_ptr == controller.picgrid.width {
                column_ptr = 0;
                row_ptr += 1;
            }
        }
    }
}
