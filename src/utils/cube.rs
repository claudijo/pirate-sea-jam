pub fn displaced_liquid_volume(side: f32, vertical_position: f32, water_height: f32) -> f32 {
    let max_submerged_depth = side / 2.;

    // Above surface
    if vertical_position >= water_height + max_submerged_depth {
        return 0.;
    }

    let volume = side.powi(3);

    // At maximum depth
    if vertical_position <= water_height - max_submerged_depth {
        return volume;
    }

    // Partially submerged
    volume * (vertical_position - max_submerged_depth - water_height) / side * -1.
}

#[cfg(test)]
mod tests {
    use super::*;

    // No tide
    #[test]
    fn above_surface_no_tide() {
        let displayced_volume = displaced_liquid_volume(4., 2., 0.);
        assert_eq!(displayced_volume, 0.);
    }

    #[test]
    fn mostly_above_surface_no_tide() {
        let displayced_volume = displaced_liquid_volume(4., 1., 0.);
        assert_eq!(displayced_volume, 16.);
    }

    #[test]
    fn half_submerged_no_tide() {
        let displayced_volume = displaced_liquid_volume(4., 0., 0.);
        assert_eq!(displayced_volume, 32.);
    }
    #[test]
    fn mostly_submerged_no_tide() {
        let displayced_volume = displaced_liquid_volume(4., -1., 0.);
        assert_eq!(displayced_volume, 48.);
    }

    #[test]
    fn fully_submerged_no_tide() {
        let displayced_volume = displaced_liquid_volume(4., -2., 0.);
        assert_eq!(displayced_volume, 64.);
    }

    // High tide
    #[test]
    fn above_surface_high_tide() {
        let displayced_volume = displaced_liquid_volume(4., 3., 1.);
        assert_eq!(displayced_volume, 0.);
    }

    #[test]
    fn mostly_above_surface_high_tide() {
        let displayced_volume = displaced_liquid_volume(4., 2., 1.);
        assert_eq!(displayced_volume, 16.);
    }

    #[test]
    fn half_submerged_high_tide() {
        let displayced_volume = displaced_liquid_volume(4., 1., 1.);
        assert_eq!(displayced_volume, 32.);
    }
    #[test]
    fn mostly_submerged_high_tide() {
        let displayced_volume = displaced_liquid_volume(4., 0., 1.);
        assert_eq!(displayced_volume, 48.);
    }

    #[test]
    fn fully_submerged_high_tide() {
        let displayced_volume = displaced_liquid_volume(4., -1., 1.);
        assert_eq!(displayced_volume, 64.);
    }

    // Low tide
    #[test]
    fn above_surface_low_tide() {
        let displayced_volume = displaced_liquid_volume(4., 1., -1.);
        assert_eq!(displayced_volume, 0.);
    }

    #[test]
    fn mostly_above_surface_low_tide() {
        let displayced_volume = displaced_liquid_volume(4., 0., -1.);
        assert_eq!(displayced_volume, 16.);
    }

    #[test]
    fn half_submerged_low_tide() {
        let displayced_volume = displaced_liquid_volume(4., -1., -1.);
        assert_eq!(displayced_volume, 32.);
    }
    #[test]
    fn mostly_submerged_low_tide() {
        let displayced_volume = displaced_liquid_volume(4.,-2., -1.);
        assert_eq!(displayced_volume, 48.);
    }

    #[test]
    fn fully_submerged_low_tide() {
        let displayced_volume = displaced_liquid_volume(4., -3., -1.);
        assert_eq!(displayced_volume, 64.);
    }
    
}
