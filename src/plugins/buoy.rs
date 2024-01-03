use crate::components::ship::PlayerShip;
use crate::resources::wave::Wave;
use bevy::prelude::*;

#[derive(Component)]
struct Buoy;

fn spawn_buoy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Icosphere::default().try_into().unwrap()),
            material: materials.add(Color::ORANGE_RED.into()),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Buoy,
    ));
}

fn keep_at_water_level(
    ship_query: Query<&Transform, (With<PlayerShip>, Without<Buoy>)>,
    mut buoy_query: Query<&mut Transform, With<Buoy>>,
    wave: Res<Wave>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();

    for ship_transform in &ship_query {
        for mut buoy_transform in &mut buoy_query {
            let water_height = wave.surface_height(
                buoy_transform.translation - ship_transform.translation,
                elapsed_time,
            );
            buoy_transform.translation.y = water_height;
        }
    }
}

pub struct BuoyPlugin;

impl Plugin for BuoyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_buoy);
        app.add_systems(Update, keep_at_water_level);
    }
}
