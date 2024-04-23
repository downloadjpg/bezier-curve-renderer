pub struct ControlPoint {
    // position of the control point
    pub position: [f64; 2], // [f64; 2] is an array of two f64's
    // might also need tangents?

}
pub struct Bezier {
    // collection of control points
    pub control_points: Vec<[ControlPoint]>
    gl
}

impl Bezier {
   // we need a function to draw the curve, as well as some functions to add/modify the control points.
   // should the modification functions return a new Bezier object, or modify the existing one?
   // should they even be a part of the bezier struct?
   // we'd likely want a 'control point' struct that handles user input, dragging, clicking, etc.
    // and then the bezier struct would just be a collection of control points.

    fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;

        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // Draw a line between each pair of control points
        for point in self.control_points.windows(2) {
            let (x1, y1) = (point[0].position[0], point[0].position[1]);
            let (x2, y2) = (point[1].position[0], point[1].position[1]);

            gl.draw(args.viewport(), |c, gl| {
                line(RED, 1.0, [x1, y1, x2, y2], c.transform, gl);
            });
        }
    }
}