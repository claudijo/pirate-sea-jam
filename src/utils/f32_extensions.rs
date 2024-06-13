use bevy::math::FloatExt;
use std::f32::consts::E;

pub trait F32Ext {
    #[allow(dead_code)]
    fn damp(self, rhs: Self, lambda: f32, delta_time: f32) -> Self;
    fn random_variation(self, rhs: Self) -> Self;
}
impl F32Ext for f32 {
    // lambda has range between `0` and infinity, will approach rhs
    // See https://www.rorydriscoll.com/2016/03/07/frame-rate-independent-damping-using-lerp/
    fn damp(self, rhs: Self, lambda: f32, delta_time: f32) -> Self {
        self.lerp(rhs, 1. - E.powf(-lambda * delta_time))
    }

    fn random_variation(self, rhs: Self) -> Self {
        self + rhs * (2.0 * rand::random::<f32>() - 1.0)
    }
}
