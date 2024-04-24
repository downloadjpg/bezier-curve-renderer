use opengl_graphics::{GlGraphics, GLSL};
use piston::input::RenderArgs;


pub struct CubicBezier {
    // collection of control points
    control_points: [[f64; 2]; 4],
    selected_point: Option<usize>, // index of a control point actively clicked on
}
impl CubicBezier { // Initialization
    const BOX_SIZE: f64 = 10.0;
    const BOX_COLOR: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
    const LINE_WIDTH: f64 = 1.0;
    const LINE_COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

    pub fn new() -> CubicBezier {
        // return a new CubicBezier curve with 4 control points near the center of the screen
        let p = [
            [50.0, 50.0],
            [100.0, 100.0],
            [150.0, 100.0],
            [200.0, 50.0],
        ];
        CubicBezier {
            control_points: p,
            selected_point: None,
        }
    }
}
impl CubicBezier { // Rendering
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        self.render_cage(args, gl);
        self.render_curve(args, gl);
        self.render_control_points(args, gl);

    }

    fn render_control_points(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        // draw the control points as boxes
        use graphics::*;
        for pos in &self.control_points {
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

    fn render_curve(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        // generate a list of points on the curve
        use graphics::*;
        const NUM_POINTS: usize = 100;
        let mut points = [[0.0; 2]; NUM_POINTS];
        for i in 0..NUM_POINTS {
            let t = i as f64 / (NUM_POINTS - 1) as f64;
            points[i] = self.de_casteljaus(t);
        }
        // draw the curve
        for segment in points.windows(2) {
            let p1 = segment[0];
            let p2 = segment[1];
            gl.draw(args.viewport(), |c, gl| {
                line(Self::LINE_COLOR, Self::LINE_WIDTH, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
            });
        }
    }

    fn render_cage(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        // render the control cage
        use graphics::*;
        //draw a line between every control point (cage)
        for segment in self.control_points.windows(2) {
            let p1 = segment[0];
            let p2 = segment[1];
            gl.draw(args.viewport(), |c, gl| {
                line(Self::LINE_COLOR, Self::LINE_WIDTH, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
            });
        }
    }
}

impl CubicBezier { // Interaction
    pub fn click(&mut self, x: f64, y: f64) {
        // check if the click is on a control point
        for (i, pos) in self.control_points.iter().enumerate() {
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
            self.control_points[i] = [x,y];
        }
    }
}

// CubicBezier functions
impl CubicBezier {
    // Assume the curve is a cubic CubicBezier curve for now.

    // Return a point on the curve at time t
    // Blending function format.
    pub fn point(&self, t: f64) -> [f64; 2] {
        let t3 = t.powf(3.0);
        let t2 = t.powf(2.0);
    
        let p0 = self.control_points[0];
        let p1 = self.control_points[1];
        let p2 = self.control_points[2];
        let p3 = self.control_points[3];
    
        [
            ( -t3 + 3.0*t2 - 3.0*t + 1.0 ) * p0[0] +
            ( 3.0*t3 - 6.0*t2 + 3.0*t     ) * p1[0] +
            ( -3.0*t3 + 3.0*t2            ) * p2[0] +
            ( t3                           ) * p3[0],
    
            ( -t3 + 3.0*t2 - 3.0*t + 1.0 ) * p0[1] +
            ( 3.0*t3 - 6.0*t2 + 3.0*t     ) * p1[1] +
            ( -3.0*t3 + 3.0*t2            ) * p2[1] +
            ( t3                           ) * p3[1],
        ]
    }

    pub fn de_casteljaus(&self, t: f64) -> [f64; 2] {
        let p0: [f64; 2] = self.control_points[0];
        let p1 = self.control_points[1];
        let p2 = self.control_points[2];
        let p3 = self.control_points[3];

        let q0 = [
            p0[0] + t * (p1[0] - p0[0]),
            p0[1] + t * (p1[1] - p0[1]),
        ];
        let q1 = [
            p1[0] + t * (p2[0] - p1[0]),
            p1[1] + t * (p2[1] - p1[1]),
        ];
        let q2 = [
            p2[0] + t * (p3[0] - p2[0]),
            p2[1] + t * (p3[1] - p2[1]),
        ];
        
        let r0 = [
            q0[0] + t * (q1[0] - q0[0]),
            q0[1] + t * (q1[1] - q0[1]),
        ];
        let r1 = [
            q1[0] + t * (q2[0] - q1[0]),
            q1[1] + t * (q2[1] - q1[1]),
        ];
        
        [
            r0[0] + t * (r1[0] - r0[0]),
            r0[1] + t * (r1[1] - r0[1]),
            ]
        // Could put this all in to a struct.. so we can render each subdivision level.
        // Level 0 = [p0, p1, p2, p3]
        // Level 1 = [q0, q1, q2]
        // Level 2 = [r0, r1]
        // Level 3 = [s0]
    }

}