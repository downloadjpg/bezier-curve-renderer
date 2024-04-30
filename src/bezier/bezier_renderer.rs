extern crate opengl_graphics;
use crate::bezier::BezierCurve;
use crate::color_palettes::Palette;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

// This struct is responsible for rendering the Bezier curves.
// It has a time parameter that can be used to animate the curves, as well as a set of display parameters.
// .... I have not yet implemented the display parameters.
pub struct BezierRenderer {
    pub time: f64,
    pub params: DisplayParameters,
}

pub struct DisplayParameters {
    pub draw_cage: bool,
    pub draw_curve: bool,
    pub draw_control_points: bool,
    pub box_size: f64,
    palette: Palette,
}

impl BezierRenderer {
    pub fn new() -> BezierRenderer {
        BezierRenderer {
            time: 0.5,
            params: DisplayParameters {
                draw_cage: true,
                draw_curve: true,
                draw_control_points: true,
                box_size: 5.0,
                palette: Palette::default(),
            },
        }
    }

    pub fn update_time(&mut self, delta: f64) {
        self.time = (self.time + delta).clamp(0.0, 1.0);
    }

    pub fn render(&self, curves: &Vec<BezierCurve>, args: &RenderArgs, gl: &mut GlGraphics) {
        if curves.len() == 0 {
            return;
        }
        for curve in curves {
            self.render_cage(curve, args, gl); // draws the control points and lines between them. uses de Casteljau's algorithm.
            self.render_curve(curve, args, gl); // draws the curve itself, uses basis function form.
        }
    }

    // Rendering is composed of three parts:
    // - de Casteljau's subdivision level points.
    // - Lines between the de Casteljau points.
    // - The curve itself.

    // This one is silly. Nabs a nice looking color from my strange color palette code.
    fn get_color(&self, subdivision: usize, max_subdivision: usize) -> [f32; 4] {
        // returns white if the subdivision is at level 0 or max, else it cycles between three colors.
        let colors = &self.params.palette.curve_colors;
        let white = self.params.palette.white.to_rgba();
        if subdivision == 0 || subdivision == max_subdivision {
            return white;
        } else {
            return colors[subdivision % colors.len()].to_rgba();
        }
    }

    // This draws everything but the actual curve, including the visualization of the de Casteljau's algorithm.
    fn render_cage(&self, curve: &BezierCurve, args: &RenderArgs, gl: &mut GlGraphics) {
        for subdivision in 0..curve.control_points.len() {
            let points = curve.de_casteljaus(self.time, subdivision);
            let color = self.get_color(subdivision, curve.control_points.len() - 1);
            // special case for control points, draw boxes instead of circles.
            if subdivision == 0 {
                self.draw_control_points(&points, args, gl);
                self.draw_lines(&points, color, args, gl);
                continue;
            }
            // set color, thickness, based on subdivision level
            self.draw_lines(&points, color, args, gl);
            self.draw_points(&points, color, args, gl);
        }
    }

    // Draws the curve itself. Uses the blending function form.
    fn render_curve(&self, curve: &BezierCurve, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        const LINE_WIDTH: f64 = 1.0;
        let line_color = [1.0, 1.0, 1.0, 1.0];

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
                line(
                    line_color,
                    LINE_WIDTH,
                    [p1[0], p1[1], p2[0], p2[1]],
                    c.transform,
                    gl,
                );
            });
        }
    }

    // Draws lines between each point in the given list.
    fn draw_lines(
        &self,
        points: &[[f64; 2]],
        color: [f32; 4],
        args: &RenderArgs,
        gl: &mut GlGraphics,
    ) {
        // draws a line between each point
        use graphics::*;
        const LINE_WIDTH: f64 = 0.5;
        for segment in points.windows(2) {
            let p1 = segment[0];
            let p2 = segment[1];
            gl.draw(args.viewport(), |c, gl| {
                line(
                    color,
                    LINE_WIDTH,
                    [p1[0], p1[1], p2[0], p2[1]],
                    c.transform,
                    gl,
                );
            });
        }
    }

    // Draws a circle at each point in the given list.
    fn draw_points(
        &self,
        points: &[[f64; 2]],
        color: [f32; 4],
        args: &RenderArgs,
        gl: &mut GlGraphics,
    ) {
        // draws a circle at each point
        use graphics::*;
        let radius: f64;

        if points.len() == 1 {
            radius = 4.0; // little hack to make the final point bigger
        } else {
            radius = 2.0;
        }

        for pos in points {
            gl.draw(args.viewport(), |c, gl| {
                ellipse(
                    color,
                    [pos[0] - radius, pos[1] - radius, radius * 2.0, radius * 2.0],
                    c.transform,
                    gl,
                );
            });
        }
    }

    // Special case draw function for making little boxes for control points.
    fn draw_control_points(&self, points: &[[f64; 2]], args: &RenderArgs, gl: &mut GlGraphics) {
        // draw the control points as boxes
        use graphics::*;
        let box_size = self.params.box_size;
        for pos in points {
            let rect = [
                pos[0] - box_size / 2.0,
                pos[1] - box_size / 2.0,
                box_size,
                box_size,
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
