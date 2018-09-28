//! PictureGrid view.

use piston_window::character::CharacterCache;
use piston_window::context::Context;
use piston_window::line::Line;
use piston_window::rectangle::{Border, Rectangle};
use piston_window::text::Text;
use piston_window::types::Color;
use piston_window::{Graphics, Transformed};

use picgrid::{CellState, PictureGrid};
use PictureGridController;

/// Stores picgrid view settings.
pub struct PictureGridViewSettings {
    /// (x,y) position of upper left corner of grid
    pub position: [f64; 2],
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
            position: [30.0; 2],
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
    pub fn draw<C, G>(
        &self,
        controller: &PictureGridController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
    {
        let ref settings = self.settings;
        let mut cell_size = settings.cell_size;
        if let Some(vp) = c.viewport {
            let hcell = ((vp.rect[2] as f64) - (settings.position[0] * 2.0))
                / (controller.picgrid.width as f64);
            let vcell = ((vp.rect[3] as f64) - (settings.position[1] * 2.0))
                / (controller.picgrid.height as f64);

            cell_size = f64::min(hcell, vcell);
        }

        let grid_rect = [
            settings.position[0],
            settings.position[1],
            (cell_size * f64::from(controller.picgrid.width)) + settings.grid_border_width,
            (cell_size * f64::from(controller.picgrid.height)) + settings.grid_border_width,
        ];

        // outer grid border
        Rectangle::new_border(
            settings.grid_border_color,
            (settings.grid_border_width / 2.0).into(),
        ).draw(grid_rect, &c.draw_state, c.transform, g);

        // grid cells
        let cell_border = Border {
            color: settings.grid_border_color,
            radius: (settings.cell_border_width / 2.0).into(),
        };
        let cell_unsolved =
            Rectangle::new(settings.cell_unsolved_background_color).border(cell_border);
        let cell_solved_shaded =
            Rectangle::new(settings.cell_solved_shaded_background_color).border(cell_border);
        let cell_solved_unshaded =
            Rectangle::new(settings.cell_solved_unshaded_background_color).border(cell_border);
        let cell_solved_unshaded_x = Line::new(
            settings.cell_solved_unshaded_hint_text_color,
            settings.cell_border_width,
        );
        let mut column_ptr: u16 = 0;
        let mut row_ptr: u16 = 0;
        let mut cell_rect = [0.0, 0.0, cell_size, cell_size];
        let grid_origin = [
            grid_rect[0] + f64::from(settings.grid_border_width / 2.0),
            grid_rect[1] + f64::from(settings.grid_border_width / 2.0),
        ];
        let cell_hint_text_size = (cell_size * 0.75) as u32;

        for state in &controller.picgrid.cells {
            cell_rect[0] = grid_origin[0] + (f64::from(column_ptr) * cell_size);
            cell_rect[1] = grid_origin[1] + (f64::from(row_ptr) * cell_size);

            let text_transform = c.transform.trans(
                cell_rect[0] + (cell_size * 0.30),
                cell_rect[1] + (cell_size * 0.75),
            );

            match state {
                CellState::Unsolved(value) => {
                    cell_unsolved.draw(cell_rect, &c.draw_state, c.transform, g);
                    if *value < PictureGrid::EMPTY {
                        Text::new_color(
                            settings.cell_unsolved_hint_text_color,
                            cell_hint_text_size,
                        ).draw(&value.to_string(), glyphs, &c.draw_state, text_transform, g)
                        .ok();
                    }
                }
                CellState::Shaded(value) => {
                    cell_solved_shaded.draw(cell_rect, &c.draw_state, c.transform, g);
                    if *value < PictureGrid::EMPTY {
                        Text::new_color(
                            settings.cell_solved_shaded_hint_text_color,
                            cell_hint_text_size,
                        ).draw(&value.to_string(), glyphs, &c.draw_state, text_transform, g)
                        .ok();
                    }
                }
                CellState::Unshaded(value) => {
                    cell_solved_unshaded.draw(cell_rect, &c.draw_state, c.transform, g);
                    if *value < PictureGrid::EMPTY {
                        Text::new_color(
                            settings.cell_solved_unshaded_hint_text_color,
                            cell_hint_text_size,
                        ).draw(&value.to_string(), glyphs, &c.draw_state, text_transform, g)
                        .ok();
                    }

                    // draw "X"
                    let padding_percent = 0.2;
                    let left = cell_rect[0] + (cell_rect[2] * padding_percent);
                    let right = cell_rect[0] + cell_rect[2] - (cell_rect[2] * padding_percent);
                    let top = cell_rect[1] + (cell_rect[3] * padding_percent);
                    let bottom = cell_rect[1] + cell_rect[3] - (cell_rect[3] * padding_percent);
                    cell_solved_unshaded_x.draw(
                        [left, top, right, bottom],
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                    cell_solved_unshaded_x.draw(
                        [left, bottom, right, top],
                        &c.draw_state,
                        c.transform,
                        g,
                    );
                }
            }

            column_ptr += 1;
            if column_ptr == controller.picgrid.width {
                column_ptr = 0;
                row_ptr += 1;
            }
        }
    }
}
