pub struct BezierCurve {
    // collection of 4 control points
    pub control_points: Vec<[f64; 2]>,
    pub selected_point: Option<usize>, // index of a control point actively clicked on, used for dragging
}
impl BezierCurve { // Initialization
    pub fn new() -> BezierCurve {
        // return a new CubicBezier curve with 4 control points near the center of the screen
        let p = vec![
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

    pub fn with_control_points(control_points: Vec<[f64; 2]>) -> Self {
        Self {
            control_points,
            ..Self::new()
        }
    }
}


impl BezierCurve { // Interaction

    const GRID_SIZE: f64 = 10.0;

    pub fn add_point(&mut self, x: f64, y: f64) {
        // add a new control point at the specified location
        self.control_points.push([x, y]);
    }
    
    pub fn click(&mut self, x: f64, y: f64) -> bool {
        // check if the click is on a control point
        for (i, pos) in self.control_points.iter().enumerate() {
            let rect = [
                pos[0] - Self::GRID_SIZE / 2.0,
                pos[1] - Self::GRID_SIZE / 2.0,
                Self::GRID_SIZE, Self::GRID_SIZE
            ];
            if x >= rect[0] && x <= rect[0] + rect[2] 
            && y >= rect[1] && y <= rect[1] + rect[3] {
                self.selected_point = Some(i);
               return true
            }
        }
        false
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
    
    pub fn de_casteljaus(&self, t: f64, i: usize)  -> Vec<[f64; 2]> {
       // returns the points used in the de Casteljau algorithm at time t, and subdivision i.
       // i = 0 returns the control points.
       // i = 1 returns interpolation between the control points, etc.\
       // i = n-1 returns the final point on the curve.
       
       // base case: i = 0
       if i == 0 {
            return self.control_points.clone();
       }

       // recursive case: i > 0
       let mut points = vec![];
       for segment in self.de_casteljaus(t, i-1).windows(2) {
           let p1 = segment[0];
           let p2 = segment[1];
            points.push([
                (1.0 - t) * p1[0] + t * p2[0],
                (1.0 - t) * p1[1] + t * p2[1],
            ]);
        }       
        points
    }

    pub fn final_point(&self, t: f64) -> [f64; 2] {
        // returns the final point on the curve at time t
        self.de_casteljaus(t, self.control_points.len() - 1)[0]
    }

}