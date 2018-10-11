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
    /// color of current cell being processed by algorithm
    pub cell_current_color: Color,
}

impl PictureGridViewSettings {
    /// Creates new picgrid view settings.
    pub fn new() -> PictureGridViewSettings {
        PictureGridViewSettings {
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
            cell_current_color: [1.0, 0.0, 0.0, 1.0],
        }
    }
}

/// Stores visual information about a picture grid
pub struct PictureGridView {
    /// Stores picgrid view settings.
    pub settings: PictureGridViewSettings,
    /// Calculated cell size
    pub cell_size: f64,
    /// Calculated position/size of grid
    pub grid_rect: [f64; 4],
}

impl PictureGridView {
    /// Creates a new picgrid view.
    pub fn new(settings: PictureGridViewSettings) -> PictureGridView {
        PictureGridView {
            settings: settings,
            cell_size: 1.0,
            grid_rect: [1.0; 4],
        }
    }

    /// Draw picture grid.
    pub fn draw<C, G>(
        &mut self,
        controller: &PictureGridController,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
    {
        let ref settings = self.settings;
        self.cell_size = settings.cell_size;
        if let Some(vp) = c.viewport {
            let hcell = ((vp.rect[2] as f64) - settings.margin[0] - settings.grid_position[0])
                / (controller.picgrid.width as f64);
            let vcell = ((vp.rect[3] as f64) - (settings.margin[1] * 2.0))
                / (controller.picgrid.height as f64);

            self.cell_size = f64::min(hcell, vcell);
        }

        self.grid_rect = [
            settings.grid_position[0],
            settings.grid_position[1],
            (self.cell_size * f64::from(controller.picgrid.width)) + settings.grid_border_width,
            (self.cell_size * f64::from(controller.picgrid.height)) + settings.grid_border_width,
        ];

        // outer grid border
        Rectangle::new_border(
            settings.grid_border_color,
            (settings.grid_border_width / 2.0).into(),
        ).draw(self.grid_rect, &c.draw_state, c.transform, g);

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
        let current_cell = Rectangle::new(settings.cell_current_color);
        let mut column_ptr: u16 = 0;
        let mut row_ptr: u16 = 0;
        let mut cell_rect = [0.0, 0.0, self.cell_size, self.cell_size];
        let grid_origin = [
            self.grid_rect[0] + f64::from(settings.grid_border_width / 2.0),
            self.grid_rect[1] + f64::from(settings.grid_border_width / 2.0),
        ];
        let cell_hint_text_size = (self.cell_size * 0.75) as u32;

        for state in &controller.picgrid.cells {
            cell_rect[0] = grid_origin[0] + (f64::from(column_ptr) * self.cell_size);
            cell_rect[1] = grid_origin[1] + (f64::from(row_ptr) * self.cell_size);

            let text_transform = c.transform.trans(
                cell_rect[0] + (self.cell_size * 0.30),
                cell_rect[1] + (self.cell_size * 0.75),
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

            if let Some(pos) = &controller.cell_pos {
                if (pos[0] - 1 == column_ptr as isize) && (pos[1] == row_ptr as isize) {
                    current_cell.draw(cell_rect, &c.draw_state, c.transform, g);
                }
            }

            column_ptr += 1;
            if column_ptr == controller.picgrid.width {
                column_ptr = 0;
                row_ptr += 1;
            }
        }

        let labels = controller.messages.to_vec();
        // labels.push("".to_string());
        // labels.push(format!("Cell Size: {:?}", &self.cell_size));
        // labels.push(format!("Cursor: {:?}", &controller.cursor_pos[..]));
        // labels.push(format!(
        //     "Grid Cell: {:?}",
        //     &match controller.cell_pos {
        //         Some(pos) => format!("{:?}", &pos[..]),
        //         None => "---".to_string(),
        //     }
        // ));

        let label_graphic = Text::new_color(settings.label_color, settings.label_size);
        let mut label_offset = settings.label_size as f64;
        for label in labels.iter() {
            label_graphic
                .draw(
                    &label,
                    glyphs,
                    &c.draw_state,
                    c.transform.trans(
                        settings.controls_position[0],
                        settings.controls_position[1] + label_offset,
                    ),
                    g,
                ).ok();
            label_offset += (settings.label_size as f64) * 1.5;
        }
    }
}
