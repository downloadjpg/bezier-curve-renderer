use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;


pub struct ControlPoint {
    // position of the control point
    pub position: [f64; 2], // [f64; 2] is an array of two f64's
    // might also need tangents?

}
pub struct Bezier {
    // collection of control points
    control_points: Vec<ControlPoint>,
    selected_point: Option<usize>, // index of a control point actively clicked on
}
impl Bezier { // Initialization
    const BOX_SIZE: f64 = 10.0;
    const BOX_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const LINE_WIDTH: f64 = 1.0;
    const LINE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    pub fn new() -> Bezier {
        // return a new Bezier curve with 4 control points near the center of the screen
        let p = vec![
            ControlPoint {
                position: [100.0, 100.0],
            },
            ControlPoint {
                position: [200.0, 100.0],
            },
            ControlPoint {
                position: [200.0, 200.0],
            },
            ControlPoint {
                position: [100.0, 200.0],
            },
        ];
        Bezier {
            control_points: p,
            selected_point: None,
        }
    }
}
impl Bezier { // Rendering
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {

        use graphics::*;
        
        // draw a line between every control point
        for i in 0..(self.control_points.len() - 1) {
            let j = i + 1;
            let p1 = self.control_points[i].position;
            let p2 = self.control_points[j].position;
            gl.draw(args.viewport(), |c, gl| {
                line(Self::LINE_COLOR, Self::LINE_WIDTH, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
            });
        }

        // draw the control points as boxes
        for cp in &self.control_points {
            let pos = cp.position;
            let rect = [
                pos[0] - Self::BOX_SIZE / 2.0,
                pos[1] - Self::BOX_SIZE / 2.0,
                Self::BOX_SIZE, Self::BOX_SIZE
            ];
            
            gl.draw(args.viewport(), |c, gl| {
                rectangle(
                    Self::BOX_COLOR,
                    rect, // x, y, width, height
                    c.transform,
                    gl,
                );
            });
        }
    }
}
impl Bezier { // Interaction
    pub fn click(&mut self, x: f64, y: f64) {
        // check if the click is on a control point
        for (i, cp) in self.control_points.iter().enumerate() {
            let pos = cp.position;
            let rect = [
                pos[0] - Self::BOX_SIZE / 2.0,
                pos[1] - Self::BOX_SIZE / 2.0,
                Self::BOX_SIZE, Self::BOX_SIZE
            ];
            if x >= rect[0] && x <= rect[0] + rect[2] && y >= rect[1] && y <= rect[1] + rect[3] {
                self.selected_point = Some(i);
                return;
            }
        }
    }

    pub fn release(&mut self) {
        self.selected_point = None;
    }

    pub fn drag(&mut self, x: f64, y: f64) {
        if let Some(i) = self.selected_point {
            self.control_points[i].position = [x,y];
        }
    }
}