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

use crate::color_palettes::{Palette, NORD, FLAT};
use bezier::{BezierCurve, BezierRenderer};

// 
use piston::{Button, MouseButton, MouseCursorEvent, PressEvent, ReleaseEvent};
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    curves: Vec<BezierCurve>,
    selected_curve: Option<usize>,
    renderer: BezierRenderer,
    palette: Palette
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let color = self.palette.background.to_rgba();
        self.gl.draw(args.viewport(), |c: Context, gl| {
            // Clear the screen.
            clear(color, gl);
        
        });
        // Render the curve.
        self.renderer.render(&self.curves, args, &mut self.gl); 

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
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [200, 200])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        // bezier curve with 4 points.
        curves: vec![BezierCurve::new()],
        palette: NORD,
        renderer: BezierRenderer::new(),
        selected_curve: None,
    };

    let mut events = Events::new(EventSettings::new());
    let mut cursor = [0.0, 0.0];
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        // Handle mouse input
        // if the left mouse button is pressed, call self.my_curve.click(x,y) with the mouse's current position
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            for (i, curve) in app.curves.iter_mut().enumerate() {
                let success = curve.click(cursor[0], cursor[1]);
                if success {
                    app.selected_curve = Some(i);
                    break;
                }
            }
        }

        if let Some(Button::Mouse(MouseButton::Right)) = e.press_args() {
            if let Some(selected_curve_index) = app.selected_curve {
                let selected = &mut app.curves[selected_curve_index];
                selected.add_point(cursor[0], cursor[1]);
            }
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                piston::Key::Space => app.curves.push(BezierCurve::new()),
                _ => {}
            }
        }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            for curve in app.curves.iter_mut() {
                curve.release();
            }
        }

        if let Some(args) = e.mouse_cursor_args() {
            cursor = args;
            for curve in app.curves.iter_mut() {
                curve.drag(args[0], args[1]);
            }
        }
    }
}
