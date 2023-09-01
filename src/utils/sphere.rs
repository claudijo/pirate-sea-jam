use std::f32::consts::PI;

pub fn cross_section_area(radius: f32) -> f32 {
    PI * radius.powi(2)
}

pub fn volume(radius: f32) -> f32 {
    4. / 3. * PI * radius.powi(3)
}

pub fn partial_volume(radius: f32, height: f32) -> f32 {
    PI / 3. * (3. * height.powi(2) * radius - height.powi(3))
}

pub fn displaced_liquid_volume(radius: f32, vertical_position: f32, water_height: f32) -> f32 {
    // Above surface
    if vertical_position >= water_height + radius {
        return 0.;
    }

    if vertical_position <= water_height - radius {
        // Sphere volume
        return volume(radius);
    }

    // Partially submerged
    partial_volume(radius, water_height - vertical_position + radius)
}

#[cfg(test)]
mod tests {
    use super::*;

    // No tide
    #[test]
    fn above_surface_no_tide() {
        let displayced_volume = displaced_liquid_volume(2., 2., 0.);
        assert_eq!(displayced_volume, 0.);
    }

    #[test]
    fn mostly_above_surface_no_tide() {
        let displayced_volume = displaced_liquid_volume(2., 1., 0.);
        assert_eq!(displayced_volume, 5.2359877);
    }

    #[test]
    fn half_submerged_no_tide() {
        let displayced_volume = displaced_liquid_volume(2., 0., 0.);
        assert_eq!(displayced_volume, 16.755161);
    }

    #[test]
    fn mostly_submerged_no_tide() {
        let displayced_volume = displaced_liquid_volume(2., -1., 0.);
        assert_eq!(displayced_volume, 28.274334);
    }

    #[test]
    fn fully_submerged_no_tide() {
        let displayced_volume = displaced_liquid_volume(2., -2., 0.);
        assert_eq!(displayced_volume, 33.510323);
    }

    // High tide
    #[test]
    fn above_surface_high_tide() {
        let displayced_volume = displaced_liquid_volume(2., 3., 1.);
        assert_eq!(displayced_volume, 0.);
    }

    #[test]
    fn mostly_above_surface_high_tide() {
        let displayced_volume = displaced_liquid_volume(2., 2., 1.);
        assert_eq!(displayced_volume, 5.2359877);
    }

    #[test]
    fn half_submerged_high_tide() {
        let displayced_volume = displaced_liquid_volume(2., 1., 1.);
        assert_eq!(displayced_volume, 16.755161);
    }

    #[test]
    fn mostly_submerged_high_tide() {
        let displayced_volume = displaced_liquid_volume(2., 0., 1.);
        assert_eq!(displayced_volume, 28.274334);
    }

    #[test]
    fn fully_submerged_high_tide() {
        let displayced_volume = displaced_liquid_volume(2., -1., 1.);
        assert_eq!(displayced_volume, 33.510323);
    }

    // Low tide
    #[test]
    fn above_surface_low_tide() {
        let displayced_volume = displaced_liquid_volume(2., 1., -1.);
        assert_eq!(displayced_volume, 0.);
    }

    #[test]
    fn mostly_above_surface_low_tide() {
        let displayced_volume = displaced_liquid_volume(2., 0., -1.);
        assert_eq!(displayced_volume, 5.2359877);
    }

    #[test]
    fn half_submerged_low_tide() {
        let displayced_volume = displaced_liquid_volume(2., -1., -1.);
        assert_eq!(displayced_volume, 16.755161);
    }

    #[test]
    fn mostly_submerged_low_tide() {
        let displayced_volume = displaced_liquid_volume(2., -2., -1.);
        assert_eq!(displayced_volume, 28.274334);
    }

    #[test]
    fn fully_submerged_low_tide() {
        let displayced_volume = displaced_liquid_volume(2., -3., -1.);
        assert_eq!(displayced_volume, 33.510323);
    }
}
