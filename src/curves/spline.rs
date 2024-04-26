use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

use crate::color_palettes::{Palette, FLAT, NORD};

use super::cubic_bezier::CubicBezier;
use super::DisplayParameters;

pub struct Spline {
    pub segments : Vec<CubicBezier>,
    pub display_parameters: DisplayParameters,
    pub links: Vec<Link>,
}

type Link = (usize, usize); // (start, end)

impl Spline {
    pub fn new() -> Self {
        Spline { 
            segments: Vec::new(), 
            display_parameters: DisplayParameters {
                draw_cage: true,
                draw_curve: true,
                draw_control_points: true,
                draw_tangents: true,
            },
            links: Vec::new(), 
        }
    }

    pub fn new_default() -> Self {
        let mut spline = Spline {
            segments: vec![
                CubicBezier::new(),
                CubicBezier::new(),
                CubicBezier::new(),
                CubicBezier::new(),
            ],
            display_parameters: DisplayParameters {
                draw_cage: true,
                draw_curve: true,
                draw_control_points: true,
                draw_tangents: true,
            },
            links: vec![
                (0, 1),
                (1, 2),
                (2, 3),
            ],
        };
        // space out the control points according to a default spline
        // number of unique control points = 4 * number of segments - links
        // We can set the first segment as normal, then only set the first 3 control points of each segmen
        // The first control point of each segment will be set to the last control point of the previous segment

        // set the first segment
        let p = [
            [50.0, 50.0],
            [100.0, 100.0],
            [150.0, 100.0],
            [200.0, 50.0],
        ];
        spline.segments[0].control_points = p;

        // set the rest of the segments
        for i in 1..spline.segments.len() {
            spline.segments[i].control_points[0] = spline.segments[i - 1].control_points[3];
            spline.segments[i].control_points[1] = [spline.segments[i].control_points[0][0] + 50.0, spline.segments[i].control_points[0][1] + 50.0];
            spline.segments[i].control_points[2] = [spline.segments[i].control_points[1][0] + 50.0, spline.segments[i].control_points[1][1] + 0.0];
            spline.segments[i].control_points[3] = [spline.segments[i].control_points[2][0] + 50.0, spline.segments[i].control_points[2][1] - 50.0];
        }
        spline.link_segments();
        spline
    }

    pub fn render(&self, args: &RenderArgs, gl: &mut GlGraphics) {
        for segment in &self.segments {
            segment.render(args, gl);
        }
    }

    pub fn click(&mut self, x: f64, y: f64) {
        for segment in &mut self.segments {
            segment.click(x, y);
        }
    }

    pub fn release(&mut self) {
        for segment in &mut self.segments {
            segment.release();
        }
    }

    pub fn drag(&mut self, x: f64, y: f64) {
        for segment in &mut self.segments {
            segment.drag(x, y);
        }
    }

    fn link_segments(&mut self) {
        // link the end of one segment to the start of the next
        for link in &self.links {
            self.segments[link.1].control_points[0] = self.segments[link.0].control_points[3];
        }
    }
}