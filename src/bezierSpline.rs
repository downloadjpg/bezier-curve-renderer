use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::bezier::CubicBezier;

pub struct BezierSpline {
    // This is mathematically equivalent to a series of cubic Bezier curves joined end-to-end.
    curves: Vec<CubicBezier>,
}

impl BezierSpline {
    pub fn new() -> Self {
        Self {
            curves: vec![CubicBezier::new()],
        }
    }

    pub fn add_curve(&mut self) {
        // Duplicate the last curve and add it to the spline,
        // offset the new curve so that its first control point is at the last control point of the last curve.
        let last_curve = self.curves.last().unwrap();
        let offset = [
            last_curve.control_points[3][0] - last_curve.control_points[0][0],
            last_curve.control_points[3][1] - last_curve.control_points[0][1],
        ];
        
        let new_curve = CubicBezier {
            control_points: [
                [last_curve.control_points[0][0] + offset[0], last_curve.control_points[0][1] + offset[1]],
                [last_curve.control_points[1][0] + offset[0], last_curve.control_points[1][1] + offset[1]],
                [last_curve.control_points[2][0] + offset[0], last_curve.control_points[2][1] + offset[1]],
                [last_curve.control_points[3][0] + offset[0], last_curve.control_points[3][1] + offset[1]],
            ],
            selected_point: None,
        };
        self.curves.push(new_curve);
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        for curve in &self.curves {
            curve.render(args, gl);
        }
    }

    pub fn click(&mut self, x: f64, y: f64) {
        // check if the click is on a control point
        for (i, curve) in self.curves.iter_mut().enumerate() {
            curve.click(x, y);
        }
    }

    pub fn release(&mut self) {
        for curve in &mut self.curves {
            curve.release();
        }
    }

    pub fn drag(&mut self, x: f64, y: f64) {
        for curve in &mut self.curves {
            curve.drag(x, y);
        }
    }
}

