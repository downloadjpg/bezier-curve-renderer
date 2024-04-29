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
    
    fn colors(&self) -> Vec<[f32; 4]> {
        vec![
            self.params.palette.primary.to_rgba(),
            self.params.palette.secondary.to_rgba(),
            self.params.palette.tertiary.to_rgba(),
            self.params.palette.background_accent.to_rgba(),
        ]
    }

    pub fn new() -> BezierRenderer {
        BezierRenderer {
            time: 0.5,
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

    pub fn update_time(&mut self, delta: f64) {
        self.time += delta;
    }


    pub fn render(&self, curves: &Vec<BezierCurve>, args: &RenderArgs, gl: &mut GlGraphics) {
        for curve in curves {
            self.render_cage(curve, args, gl); // draws the control points and lines between them
            self.render_curve(curve, args, gl); // draws a circle on the curve at the current time
        }
    }


    // Rendering is composed of three parts:
    // - de Casteljau's subdivision level points.
    // - Lines between the de Casteljau points.
    // - The curve itself.

    fn render_cage(&self, curve: &BezierCurve, args: &RenderArgs, gl: &mut GlGraphics) {
        for subdivision in 0..curve.control_points.len() {
            let points = curve.de_casteljaus(self.time, subdivision);
            let color = self.colors()[subdivision];
            // special case for control points, draw boxes instead of circles.
            if subdivision == 0 {
                 self.draw_control_points(&points, args, gl);
                 self.draw_lines(&points, color, args, gl);
                 continue;}
            if subdivision == curve.control_points.len() - 1 {
                self.draw_control_points(&points, color, args, gl);
                continue;
            }
            // set color, thickness, based on subdivision level
            self.draw_lines(&points,color, args, gl);
            self.draw_points(&points, color,  args, gl);
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


    fn draw_lines(&self, points: &[[f64; 2]], color: [f32; 4], args: &RenderArgs, gl: &mut GlGraphics) {
        // draws a line between each point
        use graphics::*;
        const LINE_WIDTH: f64 = 0.5;
        for segment in points.windows(2) {
            let p1 = segment[0];
            let p2 = segment[1];
            gl.draw(args.viewport(), |c, gl| {
                line(color, LINE_WIDTH, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
            });
        }
    }

    fn draw_points(&self, points: &[[f64; 2]], color: [f32; 4],  args: &RenderArgs, gl: &mut GlGraphics) {
        // draws a circle at each point
        use graphics::*;
        const POINT_RADIUS: f64 = 2.0;
        for pos in points {
            gl.draw(args.viewport(), |c, gl| {
                ellipse(
                    color,
                    [pos[0] - POINT_RADIUS, pos[1] - POINT_RADIUS, POINT_RADIUS * 2.0, POINT_RADIUS * 2.0],
                    c.transform,
                    gl,
                );
            });
        }
    }

    fn draw_control_points(&self, points: &[[f64;2]], args: &RenderArgs, gl: &mut GlGraphics) {
        // draw the control points as boxes
        use graphics::*;
        let box_size = self.params.box_size;
        for pos in points {
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
}