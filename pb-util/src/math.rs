use std::f32::consts::{PI, TAU};

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
    if f.is_finite() {
        f
    } else {
        0.
    }
}

pub fn normalize_angle(angle: f32) -> f32 {
    let normalized = angle % TAU;
    if normalized > PI {
        normalized - TAU
    } else if normalized < -PI {
        normalized + TAU
    } else {
        normalized
    }
}

#[cfg(test)]
mod tests {
    use super::normalize_angle;
    use approx::relative_eq;
    use std::f32::consts::PI;

    #[test]
    fn test_zero() {
        assert!(relative_eq!(normalize_angle(0.0), 0.0));
    }

    #[test]
    fn test_pi() {
        assert!(relative_eq!(normalize_angle(PI), PI));
    }

    #[test]
    fn test_negative_pi() {
        assert!(relative_eq!(normalize_angle(-PI), -PI));
    }

    #[test]
    fn test_over_pi() {
        assert!(relative_eq!(normalize_angle(PI + 0.1), -PI + 0.1));
    }

    #[test]
    fn test_under_negative_pi() {
        assert!(relative_eq!(normalize_angle(-PI - 0.1), PI - 0.1));
    }

    #[test]
    fn test_multiple_of_two_pi() {
        assert!(relative_eq!(normalize_angle(2.0 * PI), 0.0));
        assert!(relative_eq!(normalize_angle(-2.0 * PI), 0.0));
        assert!(relative_eq!(normalize_angle(4.0 * PI), 0.0));
    }

    #[test]
    fn test_random_angles() {
        assert!(relative_eq!(normalize_angle(3.0 * PI), PI));
        assert!(relative_eq!(normalize_angle(-3.0 * PI), -PI));
        assert!(relative_eq!(normalize_angle(2.5 * PI), 0.5 * PI));
        assert!(relative_eq!(normalize_angle(-2.5 * PI), -0.5 * PI));
    }
}
