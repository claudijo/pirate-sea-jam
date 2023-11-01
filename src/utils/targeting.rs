use bevy::prelude::*;

pub fn find_closest_target<'a>(
    source_translation: &Vec3,
    target_translations: &'a Vec<Vec3>,
) -> Option<&'a Vec3> {
    let mut closest_target: Option<&Vec3> = None;
    let mut closest_distance = f32::MAX;

    for target_translation in target_translations {
        let distance = source_translation.distance(*target_translation);
        if distance < closest_distance {
            closest_distance = distance;
            closest_target = Some(target_translation);
        }
    }

    closest_target
}
