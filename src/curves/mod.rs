mod cubic_bezier;
mod spline;

pub struct DisplayParameters {
    pub draw_cage : bool,
    pub draw_curve : bool,
    pub draw_control_points : bool,
    pub draw_tangents : bool,
}

pub use self::spline::Spline;
pub use self::cubic_bezier::CubicBezier;