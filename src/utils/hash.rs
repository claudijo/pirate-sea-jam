use bevy::prelude::*;
use bevy::utils::FixedState;
use std::hash::{BuildHasher, Hash, Hasher};

pub fn hash_f32_number(value: f32) -> u64 {
    assert!(
        value.is_finite(),
        "Hashing is not stable for NaN f32 values."
    );

    let mut hasher = FixedState.build_hasher();
    value.to_bits().hash(&mut hasher);

    #[allow(clippy::manual_hash_one)]
    hasher.finish()
}

#[allow(dead_code)]
pub fn hash_vec2(value: Vec2) -> u64 {
    assert!(
        value.is_finite(),
        "Hashing is not stable for Vec2 with NaN f32 values."
    );

    let mut hasher = FixedState.build_hasher();
    value.x.to_bits().hash(&mut hasher);
    value.y.to_bits().hash(&mut hasher);

    #[allow(clippy::manual_hash_one)]
    hasher.finish()
}

pub fn hash_vec3(value: Vec3) -> u64 {
    assert!(
        value.is_finite(),
        "Hashing is not stable for Vec3 with NaN f32 values."
    );

    let mut hasher = FixedState.build_hasher();
    value.x.to_bits().hash(&mut hasher);
    value.y.to_bits().hash(&mut hasher);
    value.z.to_bits().hash(&mut hasher);

    #[allow(clippy::manual_hash_one)]
    hasher.finish()
}

pub fn hash_quat(value: Quat) -> u64 {
    assert!(
        value.is_finite(),
        "Hashing is not stable for Quaternion with NaN f32 values."
    );

    let mut hasher = FixedState.build_hasher();
    value.x.to_bits().hash(&mut hasher);
    value.y.to_bits().hash(&mut hasher);
    value.z.to_bits().hash(&mut hasher);
    value.w.to_bits().hash(&mut hasher);

    #[allow(clippy::manual_hash_one)]
    hasher.finish()
}
