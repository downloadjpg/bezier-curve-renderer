mod cubic_bezier;
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
use cubic_bezier::{CubicBezier, Spline};


// 
use piston::{Button, MouseButton, MouseCursorEvent, PressEvent, ReleaseEvent};
pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    my_curve: Spline,
    palette: Palette
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;
        let color = self.palette.to_rgba(self.palette.background);
        self.gl.draw(args.viewport(), |c: Context, gl| {
            // Clear the screen.
            clear(color, gl);
        
        });
        // Render the curve.
        self.my_curve.render(args, &mut self.gl); 
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
        my_curve: Spline::new_default(),
        palette: NORD,
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
            app.my_curve.click(cursor[0], cursor[1]);
        }

        // if let Some(Button::Keyboard(key)) = e.press_args() {
        //     match key {
        //         piston::Key::Space => app.my_curve.add_curve(),
        //         _ => {}
        //     }
        // }

        if let Some(Button::Mouse(MouseButton::Left)) = e.release_args() {
            app.my_curve.release();
        }

        if let Some(args) = e.mouse_cursor_args() {
            cursor = args;
            app.my_curve.drag(args[0], args[1]);
        }
    }
}
