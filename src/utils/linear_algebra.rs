use bevy::math::Vec3;

pub fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
    (b - a).cross(c - a).normalize().into()
}

pub fn is_facing(source_direction: Vec3, source_position: Vec3, target_position: Vec3) -> bool {
    source_direction.dot(target_position - source_position) > 0.
}

pub fn angle_between_perpendicular(vector: Vec3, normal: Vec3) -> f32 {
    (vector.dot(normal) / vector.length()).asin()
}

pub fn perpendicular_to_projection_direction(vector: Vec3, normal: Vec3) -> Vec3 {
    normal - normal.project_onto(vector)
}
