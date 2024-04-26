mod cubic_bezier;
mod spline;

pub struct DisplayParameters {
    draw_cage : bool,
    draw_curve : bool,
    draw_control_points : bool,
    draw_tangents : bool,
}

pub use self::spline::Spline;