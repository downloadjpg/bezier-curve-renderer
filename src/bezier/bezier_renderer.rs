extern crate opengl_graphics;
use crate::bezier::BezierCurve;
use crate::color_palettes::{Palette, NORD, FLAT};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::input::{RenderArgs, RenderEvent};

pub struct BezierRenderer {
    pub time: f64,
    pub params: DisplayParameters,
}

pub struct DisplayParameters {
    pub draw_cage : bool,
    pub draw_curve : bool,
    pub draw_control_points : bool,
    pub draw_tangents : bool,
    pub box_size : f64,
    palette: Palette,
}

impl BezierRenderer {
    pub fn new() -> BezierRenderer {
        BezierRenderer {
            time: 0.0,
            params: DisplayParameters {
                draw_cage: true,
                draw_curve: true,
                draw_control_points: true,
                draw_tangents: false,
                box_size: 5.0,
                palette: NORD,
            },
        }
    }
    pub fn render(&self, curves: &Vec<BezierCurve>, args: &RenderArgs, gl: &mut GlGraphics) {
        for curve in curves {
            if self.params.draw_cage { self.render_cage(curve, args, gl); }
            if self.params.draw_curve { self.render_curve(curve, args, gl); }
            if self.params.draw_control_points { self.render_control_points(curve, args, gl); }
        }

    }

    fn render_control_points(&self, curve: &BezierCurve, args: &RenderArgs, gl: &mut GlGraphics) {
        // draw the control points as boxes 
        let box_size = self.params.box_size;
        use graphics::*;
        for pos in &curve.control_points {
            let rect = [
                pos[0] - box_size / 2.0,
                pos[1] - box_size / 2.0,
                box_size, box_size
            ];
            gl.draw(args.viewport(), |c, gl| {
                rectangle(
                    self.params.palette.primary.to_rgba(),
                    rect, // x, y, width, height
                    c.transform,
                    gl,
                );
            });
        }
    }

    fn render_curve(&self, curve: &BezierCurve, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        const LINE_WIDTH: f64 = 1.0;
        let line_color = self.params.palette.secondary.to_rgba();

        // generate a list of points on the curve
        const NUM_POINTS: usize = 100;
        let mut points = [[0.0; 2]; NUM_POINTS];
        for i in 0..NUM_POINTS {
            let t = i as f64 / (NUM_POINTS - 1) as f64;
            points[i] = curve.point(t);
        }

        // draw the curve
        for segment in points.windows(2) {
            let p1 = segment[0];
            let p2 = segment[1];
            gl.draw(args.viewport(), |c, gl| {
                line(line_color, LINE_WIDTH, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
            });
        }
    }

    fn render_cage(&self, curve: &BezierCurve, args: &RenderArgs, gl: &mut GlGraphics) {
        const LINE_WIDTH: f64 = 0.5;
        let line_color = self.params.palette.tertiary.to_rgba();
        // render the control cage
        use graphics::*;
        //draw a line between every control point (cage)
        for segment in curve.control_points.windows(2) {
            let p1 = segment[0];
            let p2 = segment[1];
            gl.draw(args.viewport(), |c, gl| {
                line(line_color, LINE_WIDTH, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
            });
        }
    }

    fn draw_circle(&self, args: &RenderArgs, gl: &mut GlGraphics, x: f64, y: f64, radius: f64) {
        use graphics::*;
        let color = self.params.palette.tertiary.to_rgba();
        let circle = ellipse::circle(x, y, radius);
        gl.draw(args.viewport(), |c, gl| {
            ellipse(color, circle, c.transform, gl);
        });
    }
}