pub fn scale_into_range(
    value: f32,
    value_min: f32,
    value_max: f32,
    target_min: f32,
    target_max: f32,
) -> f32 {
    (value - value_min) / (value_max - value_min) * (target_max - target_min) + target_min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_to_larger_range() {
        let result = scale_into_range(0.5, 0., 1., 0., 10.);
        assert_eq!(result, 5.);
    }

    #[test]
    fn scale_to_smaller_range() {
        let result = scale_into_range(8., 0., 10., 0., 1.);
        assert_eq!(result, 0.8);
    }

    #[test]
    fn scale_from_negative() {
        let result = scale_into_range(1., -1., 9., 0., 1.);
        assert_eq!(result, 0.2);
    }

    #[test]
    fn scale_to_negative() {
        let result = scale_into_range(0.1, 0., 1., -10., 0.);
        assert_eq!(result, (-9.))
    }
}
