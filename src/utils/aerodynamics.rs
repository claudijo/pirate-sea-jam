use crate::utils::linear_algebra::{
    angle_between_perpendicular, perpendicular_to_projection_direction,
};
use bevy::math::Vec3;
use std::f32::EPSILON;
use std::ops::Neg;

// https://aviation.stackexchange.com/a/64637
pub fn simple_lift_coefficient(angle_of_attack: f32) -> f32 {
    (angle_of_attack * 2.).sin()
}

// https://aviation.stackexchange.com/a/64637
pub fn simple_drag_coefficient(angle_of_attack: f32) -> f32 {
    1. - (angle_of_attack * 2.).cos()
}

// https://math.stackexchange.com/a/4890320/1306679
pub fn scaled_lift_drag(relative_velocity: Vec3, surface_normal: Vec3) -> (Vec3, Vec3) {
    if relative_velocity.length().abs() <= f32::EPSILON {
        return (Vec3::ZERO, Vec3::ZERO);
    }

    let normal = if relative_velocity.dot(surface_normal) > 0. {
        surface_normal
    } else {
        surface_normal.neg()
    };

    let angle = angle_between_perpendicular(relative_velocity, normal);

    let lift = perpendicular_to_projection_direction(relative_velocity, normal).normalize()
        * simple_lift_coefficient(angle);
    let drag = relative_velocity.normalize() * simple_drag_coefficient(angle);

    (lift, drag)
}
