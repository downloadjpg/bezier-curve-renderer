use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;


pub struct ControlPoint {
    // position of the control point
    pub position: [f64; 2], // [f64; 2] is an array of two f64's
    // might also need tangents?

}
pub struct Bezier {
    // collection of control points
    pub control_points: Vec<ControlPoint>,
    pub color: [f32; 4],
}
impl Bezier {
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
            color: [1.0, 0.0, 0.0, 1.0],
        }
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
       // draw a line between every control point
        for i in 0..self.control_points.len() {
            let j = (i + 1) % self.control_points.len();
            let p1 = self.control_points[i].position;
            let p2 = self.control_points[j].position;
            gl.draw(args.viewport(), |c, gl| {
                line(self.color, 1.0, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
            });
        }
    }
}