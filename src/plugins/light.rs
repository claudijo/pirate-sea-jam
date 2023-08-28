use crate::game_state::GameState;
use bevy::prelude::*;
use core::f32::consts::PI;

pub struct LigthPlugin;

impl Plugin for LigthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_light);
    }
}

fn spawn_light(mut commands: Commands) {
    // directional 'sun' light
    commands
        .spawn(DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 32000.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 2.0, 0.0)
                .with_rotation(Quat::from_rotation_x(-PI / 4.)),
            ..default()
        })
        .insert(Name::new("Sun Light"));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.5,
    })
}
