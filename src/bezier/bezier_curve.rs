use crate::color_palettes::{Palette, FLAT, NORD};
use piston::input::{RenderArgs, RenderEvent};

pub struct BezierCurve {
    // collection of 4 control points
    pub control_points: [[f64; 2]; 4],
    pub selected_point: Option<usize>, // index of a control point actively clicked on, used for dragging
}
impl BezierCurve { // Initialization
    pub fn new() -> BezierCurve {
        // return a new CubicBezier curve with 4 control points near the center of the screen
        let p = [
            [50.0, 50.0],
            [100.0, 100.0],
            [150.0, 100.0],
            [200.0, 50.0],
        ];
        BezierCurve {
            control_points: p,
            selected_point: None,
        }
    }

    pub fn with_control_points(control_points: [[f64; 2]; 4]) -> Self {
        Self {
            control_points,
            ..Self::new()
        }
    }
}
pub struct DeCasteljauPoints {
    // de Casteljaus method uses repeated linear interpolation to find a point on the curve.
    // This struct stores the points used in the calculation, and can be used to render the intermediate steps.
    // Level 0: The control points of the curve.
    p0 : [f64; 2],
    p1 : [f64; 2],
    p2 : [f64; 2],
    p3 : [f64; 2],
    // Level 1: The points between the control points.
    q0 : [f64; 2],
    q1 : [f64; 2],
    q2 : [f64; 2],
    // Level 2: The points between the points from level 1.
    r0 : [f64; 2],
    r1 : [f64; 2],
    // Level 3: The final point on the curve.
    s0 : [f64; 2],

}


impl BezierCurve { // Interaction
    const GRID_SIZE: f64 = 10.0;

    pub fn click(&mut self, x: f64, y: f64) {
        // check if the click is on a control point
        for (i, pos) in self.control_points.iter().enumerate() {
            let rect = [
                pos[0] - Self::GRID_SIZE / 2.0,
                pos[1] - Self::GRID_SIZE / 2.0,
                Self::GRID_SIZE, Self::GRID_SIZE
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
            let nx = (x / Self::GRID_SIZE).round() * Self::GRID_SIZE;
            let ny = (y / Self::GRID_SIZE).round() * Self::GRID_SIZE;
            self.control_points[i] = [nx,ny];
        }
    }
}

// CubicBezier functions
impl BezierCurve {
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
    pub fn de_casteljaus(&self, t: f64) -> DeCasteljauPoints {
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
        
        let s0 = [
            r0[0] + t * (r1[0] - r0[0]),
            r0[1] + t * (r1[1] - r0[1])
        ];
        DeCasteljauPoints {
            p0, p1, p2, p3,
            q0, q1, q2,
            r0, r1,
            s0,
        }
        // Could put this all in to a struct.. so we can render each subdivision level.
        // Level 0 = [p0, p1, p2, p3]
        // Level 1 = [q0, q1, q2]
        // Level 2 = [r0, r1]
        // Level 3 = [s0]
    }

}