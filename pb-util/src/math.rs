use approx::relative_eq;
use bevy::math::Vec2;

pub fn line_intersection(p1: Vec2, d1: Vec2, p2: Vec2, d2: Vec2) -> Option<Vec2> {
    let cross = d1.perp_dot(d2);
    if relative_eq!(cross, 0.) {
        None
    } else {
        Some(p1 + d1 * (p2 - p1).perp_dot(d2) / cross)
    }
}

pub fn to_finite_f32_lossy(f: f32) -> f32 {
    if f.is_finite() { f } else { 0. }
}
