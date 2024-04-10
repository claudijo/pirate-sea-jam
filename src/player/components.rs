use bevy::prelude::*;

#[derive(Default, Reflect, Component, Clone, Copy)]
#[reflect(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component)]
pub struct Wheel;

#[derive(Component)]
pub struct Flag;
