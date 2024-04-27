mod cubic_bezier;
use crate::cubic_bezier::CubicBezier;
use crate::color_palettes::Palette;

struct BezierRenderer {
    curve: &CubicBezier,
    time: f64,
    params: DisplayParameters,
}

pub struct DisplayParameters {
    pub draw_cage : bool,
    pub draw_curve : bool,
    pub draw_control_points : bool,
    pub draw_tangents : bool,
    pallete: Palette,
}

impl BezierRenderer {
    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        if self.args.draw_cage { self.render_cage(args, gl); }
        if self.args.draw_curve { self.render_curve(args, gl); }
        if self.args.draw_control_points { self.render_control_points(args, gl); }

    }

    fn render_control_points(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        // draw the control points as boxes
        let box_size = self.params.box_size;
        use graphics::*;
        for pos in curve.control_points {
            let rect = [
                pos[0] - box_size / 2.0,
                pos[1] - box_size / 2.0,
                box_size, box_size
            ];
            gl.draw(args.viewport(), |c, gl| {
                rectangle(
                    self.palette.to_rgba(self.palette.primary),
                    rect, // x, y, width, height
                    c.transform,
                    gl,
                );
            });
        }
    }

    fn render_curve(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        use graphics::*;
        const LINE_WIDTH: f64 = 1.0;
        let line_color = self.palette.secondary.to_rgba();

        // generate a list of points on the curve
        const NUM_POINTS: usize = 100;
        let mut points = [[0.0; 2]; NUM_POINTS];
        for i in 0..NUM_POINTS {
            let t = i as f64 / (NUM_POINTS - 1) as f64;
            points[i] = self.point(t);
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

    fn render_cage(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        const LINE_WIDTH: f64 = 0.5;
        let line_color = self.palette.tertiary.to_rgba();
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
        let color = self.palette.tertiary.to_rgba();
        let circle = ellipse::circle(x, y, radius);
        gl.draw(args.viewport(), |c, gl| {
            ellipse(color, circle, c.transform, gl);
        });
    }

    fn render_de_casteljaus(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        // render the de Casteljau points
        use graphics::*;
        const LINE_WIDTH: f64 = 0.5;
        let line_color = self.palette.tertiary.to_rgba();
        let de_casteljau_points = curve.de_casteljaus(0.5);
        for segment in de_casteljau_points.windows(2) {
            let p1 = segment[0];
            let p2 = segment[1];
            gl.draw(args.viewport(), |c, gl| {
                line(line_color, LINE_WIDTH, [p1[0], p1[1], p2[0], p2[1]], c.transform, gl);
            });
        }
    }
}