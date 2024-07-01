use bevy::prelude::*;

#[derive(Component)]
pub struct OceanTile {
    pub offset: Vec3,
}

#[derive(Component, Reflect, Clone, Default)]
#[reflect(Component)]
pub struct ImpactPointTimer(pub Timer);

