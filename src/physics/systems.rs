pub fn update_position() {
    // object_position += object_velocity * delta_time
}

pub fn update_velocity() {
    //  A value of 0.999 might be perfect for damping (pp 50)
    // object_velocity = object_velocity*damping^delta_time + object_acceleration * delta_time
}
