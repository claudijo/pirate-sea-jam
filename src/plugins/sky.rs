use crate::components::ship::PlayerShip;
use crate::game_state::GameState;
use crate::plugins::ocean_tile::OCEAN_TILE_SIZE;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

#[derive(Component)]
pub struct Sky;

fn spawn_sky(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Sky box
    // commands.spawn((
    //     PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Box::default())),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::hex("a5cddf").unwrap(),
    //             unlit: true,
    //             cull_mode: None,
    //             ..default()
    //         }),
    //         transform: Transform::from_scale(Vec3::splat(OCEAN_TILE_SIZE * 4.)),
    //         ..default()
    //     },
    //     Sky,
    //     NotShadowCaster,
    // ));
}

fn track_player_ship_position(
    ship_query: Query<&Transform, (With<PlayerShip>, Without<Sky>)>,
    mut sky_query: Query<&mut Transform, With<Sky>>,
) {
    for ship_transform in &ship_query {
        for mut sky_transform in &mut sky_query {
            sky_transform.translation.x = ship_transform.translation.x;
            sky_transform.translation.z = ship_transform.translation.z;
        }
    }
}

pub struct SkyPlugin;

impl Plugin for SkyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_sky);

        app.add_systems(
            Update,
            (track_player_ship_position.run_if(in_state(GameState::InGame)),),
        );
    }
}
