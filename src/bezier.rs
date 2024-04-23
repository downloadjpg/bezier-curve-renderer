use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;


pub struct ControlPoint {
    // position of the control point
    pub position: [f64; 2], // [f64; 2] is an array of two f64's
    // might also need tangents?

}
pub struct Bezier {
    // collection of control points
    pub control_points: u32
}

impl Bezier {
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
    
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
    
        let p1 = [0.0, 0.0];
        let p2 = [50.0, 50.0];
    
        gl.draw(args.viewport(), |c, gl| {
            line(RED, 1.0, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
        });
    }
}