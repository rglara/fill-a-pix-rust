//! PictureGrid controller.

use std::thread;
use std::time::Duration;

use find_folder;

use piston_window::character::CharacterCache;
use piston_window::context::Context;
use piston_window::generic_event::GenericEvent;
use piston_window::line::Line;
use piston_window::rectangle::{Border, Rectangle};
use piston_window::text::Text;
use piston_window::{
    clear, Button, Filter, Glyphs, Graphics, Key, MouseButton, PistonWindow, TextureSettings,
    Transformed,
};

use picgrid::CellState;
use picgrid_view::{PictureGridView, PictureGridViewSettings};

use PictureGrid;

/// Handles events for Fill-a-Pix grid.
pub struct PictureGridController {
    /// Stores window information for GUI
    window: PistonWindow,
    /// Stores the picture grid state.
    picgrid: PictureGrid,
}

impl PictureGridController {
    /// Creates a new picgrid controller.
    pub fn new(window: PistonWindow, picgrid: PictureGrid) -> PictureGridController {
        PictureGridController {
            window: window,
            picgrid: picgrid,
        }
    }

    /// Handles primary window eventing
    pub fn event_pump(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("assets")
            .unwrap();
        let ref font = assets.join("FiraSans-Regular.ttf");
        let factory = self.window.factory.clone();
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        let mut glyphs = Glyphs::new(font, factory, texture_settings).unwrap();
        let settings = PictureGridViewSettings::new();
        let mut info = PictureGridView::new();

        use piston_window::{ButtonArgs, ButtonState, Event, Input};
        use std::sync::mpsc;

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            let event = Event::Input(Input::Button(ButtonArgs {
                state: ButtonState::Press,
                button: Button::Keyboard(Key::A),
                scancode: None,
            }));
            tx.send(event).unwrap();
            thread::sleep(Duration::from_millis(10));
        });

        let mut thread_events = rx.iter();
        while let Some(event) = self.window.next() {
            PictureGridController::event(
                &mut self.window,
                &mut self.picgrid,
                &settings,
                &mut info,
                &mut glyphs,
                &event,
            );

            if let Some(event) = thread_events.next() {
                PictureGridController::event(
                    &mut self.window,
                    &mut self.picgrid,
                    &settings,
                    &mut info,
                    &mut glyphs,
                    &event,
                );
            }
        }
    }

    /// Handles events.
    fn event<E: GenericEvent>(
        window: &mut PistonWindow,
        picgrid: &mut PictureGrid,
        settings: &PictureGridViewSettings,
        info: &mut PictureGridView,
        glyphs: &mut Glyphs,
        e: &E,
    ) {
        if let Some(cp) = e.mouse_cursor_args() {
            info.cursor_pos = cp;
            if info.cursor_pos[0] > info.grid_rect[0]
                && info.cursor_pos[1] > info.grid_rect[1]
                && info.cursor_pos[0] < (info.grid_rect[0] + info.grid_rect[2])
                && info.cursor_pos[1] < (info.grid_rect[1] + info.grid_rect[3])
            {
                // we're within the grid, so see what cell we're in (size + border width)
                info.cell_pos = Some([
                    ((info.cursor_pos[0] - info.grid_rect[0]) / (info.cell_size + 1.0)).trunc()
                        as isize,
                    ((info.cursor_pos[1] - info.grid_rect[1]) / (info.cell_size + 1.0)).trunc()
                        as isize,
                ]);
            } else {
                info.cell_pos = None;
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            if let Some(pos) = info.cell_pos {
                if let Some(cell) = picgrid.get(pos[0], pos[1]) {
                    let new_state;
                    match cell {
                        CellState::Unsolved(val) => new_state = CellState::Shaded(val),
                        CellState::Shaded(val) => new_state = CellState::Unshaded(val),
                        CellState::Unshaded(val) => new_state = CellState::Unsolved(val),
                    };
                    picgrid.set(pos[0], pos[1], new_state);
                }
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::X => {
                    PictureGridController::run_solver(window, picgrid, settings, info, glyphs, e)
                }
                _ => {
                    println!("UNHANDLED: {:?}", key);
                }
            }
        }

        PictureGridController::redraw(window, picgrid, settings, info, glyphs, e);
    }

    /// Redraw screen
    fn redraw<E: GenericEvent>(
        window: &mut PistonWindow,
        picgrid: &mut PictureGrid,
        settings: &PictureGridViewSettings,
        info: &mut PictureGridView,
        glyphs: &mut Glyphs,
        e: &E,
    ) {
        window.draw_2d(e, |context, graphics| {
            clear(settings.background_color, graphics);
            PictureGridController::draw(&picgrid, &settings, info, glyphs, &context, graphics);
        });
    }

    /// Executes solving algorithm
    fn run_solver<E: GenericEvent>(
        window: &mut PistonWindow,
        picgrid: &mut PictureGrid,
        settings: &PictureGridViewSettings,
        info: &mut PictureGridView,
        glyphs: &mut Glyphs,
        e: &E,
    ) {
        println!("Solving picture grid...");
        let mut x: isize = 0;
        let mut y: isize = 0;

        println!("Applying zeroes and nines...");
        for _index in 0..(picgrid.width * picgrid.height) {
            if let Some(cell) = picgrid.get(x, y) {
                let cell_hint = cell.hint();
                match cell_hint {
                    0 => {
                        picgrid.fill_unshaded(x, y);
                        PictureGridController::redraw(window, picgrid, settings, info, glyphs, e);
                    }
                    9 => {
                        picgrid.fill_shaded(x, y);
                        PictureGridController::redraw(window, picgrid, settings, info, glyphs, e);
                    }
                    6 => {
                        if x == 0
                            || x == (picgrid.width - 1) as isize
                            || y == 0
                            || y == (picgrid.height - 1) as isize
                        {
                            picgrid.fill_shaded(x, y);
                            PictureGridController::redraw(
                                window, picgrid, settings, info, glyphs, e,
                            );
                        }
                    }
                    4 => {
                        if (x == 0 && y == 0)
                            || (x == 0 && y == (picgrid.height - 1) as isize)
                            || (x == (picgrid.width - 1) as isize && y == 0)
                            || (x == (picgrid.width - 1) as isize
                                && y == (picgrid.height - 1) as isize)
                        {
                            picgrid.fill_shaded(x, y);
                            PictureGridController::redraw(
                                window, picgrid, settings, info, glyphs, e,
                            );
                        }
                    }
                    _ => {}
                }
                x += 1;
                if x >= picgrid.width as isize {
                    x = 0;
                    y += 1;
                }
            }
        }

        let mut num_passes = 1;
        let mut needs_pass = true;
        while needs_pass {
            needs_pass = false;
            println!("Easy pass #{}", num_passes);

            x = 0;
            y = 0;
            // check if the cell is "satisfied"
            for _index in 0..(picgrid.width * picgrid.height) {
                if let Some(cell) = picgrid.get(x, y) {
                    let cell_hint = cell.hint();
                    if cell_hint != PictureGrid::EMPTY && !picgrid.is_complete(x, y) {
                        let cell_shaded = picgrid.num_shaded(x, y);
                        let cell_unsolved = picgrid.num_unsolved(x, y);
                        if cell_hint == cell_shaded {
                            picgrid.fill_unshaded(x, y);
                            PictureGridController::redraw(
                                window, picgrid, settings, info, glyphs, e,
                            );
                            needs_pass = true;
                        } else if cell_hint == (cell_shaded + cell_unsolved) {
                            picgrid.fill_shaded(x, y);
                            PictureGridController::redraw(
                                window, picgrid, settings, info, glyphs, e,
                            );
                            needs_pass = true;
                        }
                    }
                }
                x += 1;
                if x >= picgrid.width as isize {
                    x = 0;
                    y += 1;
                }
            }
            num_passes += 1;
        }

        println!("Done with algorithm!");
    }

    /// Draw picture grid.
    fn draw<C, G>(
        picgrid: &PictureGrid,
        settings: &PictureGridViewSettings,
        info: &mut PictureGridView,
        glyphs: &mut C,
        c: &Context,
        g: &mut G,
    ) where
        C: CharacterCache,
        G: Graphics<Texture = <C as CharacterCache>::Texture>,
    {
        info.cell_size = settings.cell_size;
        if let Some(vp) = c.viewport {
            let hcell = ((vp.rect[2] as f64) - settings.margin[0] - settings.grid_position[0])
                / (picgrid.width as f64);
            let vcell =
                ((vp.rect[3] as f64) - (settings.margin[1] * 2.0)) / (picgrid.height as f64);

            info.cell_size = f64::min(hcell, vcell);
        }

        info.grid_rect = [
            settings.grid_position[0],
            settings.grid_position[1],
            (info.cell_size * f64::from(picgrid.width)) + settings.grid_border_width,
            (info.cell_size * f64::from(picgrid.height)) + settings.grid_border_width,
        ];

        // outer grid border
        Rectangle::new_border(
            settings.grid_border_color,
            (settings.grid_border_width / 2.0).into(),
        ).draw(info.grid_rect, &c.draw_state, c.transform, g);

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
        let mut cell_rect = [0.0, 0.0, info.cell_size, info.cell_size];
        let grid_origin = [
            info.grid_rect[0] + f64::from(settings.grid_border_width / 2.0),
            info.grid_rect[1] + f64::from(settings.grid_border_width / 2.0),
        ];
        let cell_hint_text_size = (info.cell_size * 0.75) as u32;

        for state in &picgrid.cells {
            cell_rect[0] = grid_origin[0] + (f64::from(column_ptr) * info.cell_size);
            cell_rect[1] = grid_origin[1] + (f64::from(row_ptr) * info.cell_size);

            let text_transform = c.transform.trans(
                cell_rect[0] + (info.cell_size * 0.30),
                cell_rect[1] + (info.cell_size * 0.75),
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
            if column_ptr == picgrid.width {
                column_ptr = 0;
                row_ptr += 1;
            }
        }

        let labels = vec![
            // format!("Cell Size: {:?}", &self.cell_size),
            // format!("Cursor: {:?}", &self.cursor_pos[..]),
            // format!(
            //     "Grid Cell: {:?}",
            //     &match self.cell_pos {
            //         Some(pos) => format!("{:?}", &pos[..]),
            //         None => "---".to_string(),
            //     }
            // ),
            // "".to_string(),
            "Press 'x' to use solving algorithm".to_string(),
        ];

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
