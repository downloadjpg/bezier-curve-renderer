mod bezier;
mod color_palettes;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent};
use piston::window::WindowSettings;

use crate::color_palettes::Palette;
use bezier::{BezierCurve, BezierRenderer};

// 
use piston::{Button, ButtonArgs, ButtonEvent, ButtonState, Key, MouseButton, MouseCursorEvent};
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    curves: Vec<BezierCurve>,
    selected_curve: Option<usize>,
    renderer: BezierRenderer,
    palette: Palette,
    button_states: [ButtonState; 2], // state of left and right arrow keys, used to scrub along time.
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        // Clear the screen.
        let color = self.palette.background.to_rgba();
        self.gl.draw(args.viewport(), |_c: Context, gl| {
            clear(color, gl);
        
        });
        //Draw grid lines
        let grid_size = 10.0;
        let thickness = 0.25;
        let color = self.palette.background_accent.to_rgba();

        self.gl.draw(args.viewport(), |c, gl| {
            let mut x = 0.0;
            let height = args.window_size[0] as f64;
            let width = args.window_size[1] as f64;
            while x < width {
                line(color, thickness, [x, 0.0, x, height as f64], c.transform, gl);
                x += grid_size;
            }
            let mut y = 0.0;
            while y < height {
                line(color, thickness, [0.0, y, width as f64, y], c.transform, gl);
                y += grid_size;
            }
        });
        // Render the curves!.
        self.renderer.render(&self.curves, args, &mut self.gl); 

    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("bezier-demo", [500, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        // bezier curve with 4 points.
        curves: vec![BezierCurve::new()],
        palette: Palette::default(),
        renderer: BezierRenderer::new(),
        selected_curve: None,
        button_states: [ButtonState::Release, ButtonState::Release],
        
    };

    let mut events = Events::new(EventSettings::new());
    let mut cursor = [0.0, 0.0];

    // Main event loop
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() { app.render(&args); }
    
        if let Some(ButtonArgs { button, state, .. }) = e.button_args() {
            handle_button_input(&mut app, button, state, cursor);
        }
        
        if let Some([x, y]) = e.mouse_cursor_args() {
            cursor = [x, y];
            if let Some(selected_curve_index) = app.selected_curve {
                let selected = &mut app.curves[selected_curve_index];
                selected.drag(x, y);
            }
        }

        match app.button_states {
            [ButtonState::Press, ButtonState::Release] => {
                // scrub backward
                app.renderer.update_time(-0.0005);
            }
            [ButtonState::Release, ButtonState::Press] => {
                // scrub forward
                app.renderer.update_time(0.0005);
            }
            _ => {}
        }
    }
    
    fn handle_button_input(app: &mut App, button: Button, state: ButtonState, cursor: [f64; 2]) {
        match button {
            Button::Mouse(MouseButton::Left) => {
                if state == ButtonState::Press {
                    handle_left_mouse_button_press(app, cursor);
                }

                else if state == ButtonState::Release {
                    for curve in &mut app.curves {
                        curve.release();
                    }
                }
            }
            Button::Mouse(MouseButton::Right) => {
                if state == ButtonState::Press {
                    handle_right_mouse_button_press(app, cursor);
                }
            }
            Button::Keyboard(key) => {
                handle_keyboard_input(app, key, state);
            }
            _ => {}
        }
    }
    
    fn handle_left_mouse_button_press(app: &mut App, cursor: [f64; 2]) {
        for (i, curve) in app.curves.iter_mut().enumerate() {
            let success = curve.click(cursor[0], cursor[1]);
            if success {
                app.selected_curve = Some(i);
                break;
            }
        }
    }
    
    fn handle_right_mouse_button_press(app: &mut App, cursor: [f64; 2]) {
        for curve in &mut app.curves {
            let remove_point_success = curve.right_click(cursor[0], cursor[1]);
            if remove_point_success {
                if curve.control_points.is_empty() {
                    app.curves.retain(|c| c.control_points.len() > 0);
                    app.selected_curve = None;
                }
                return;
            }
        }
        if let Some(selected_curve_index) = app.selected_curve {
            let selected = &mut app.curves[selected_curve_index];
            selected.add_point(cursor[0], cursor[1]);
        }
    }
    
    fn handle_keyboard_input(app: &mut App, key: Key, state: ButtonState) {
        match key {
            piston::Key::Space => {
                if state == ButtonState::Press {
                    app.curves.push(BezierCurve::new())
                }
            }
            piston::Key::Right => {
                app.button_states[0] = state;
            }
            piston::Key::Left => {
                app.button_states[1] = state;
            }
            _ => {}
        }
    }


}
