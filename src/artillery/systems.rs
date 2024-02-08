use crate::artillery::components::{ArtilleryReady, Projectile};
use crate::assets::resources::ModelAssets;
use crate::connection::systems::RollbackConfig;
use crate::inputs::fire;
use crate::player::components::Player;
use bevy::prelude::*;
use bevy_ggrs::{AddRollbackCommandExtension, PlayerInputs};
use crate::physics::components::{LinearAcceleration, LinearDamping, LinearVelocity, PhysicsBody};

// Check https://johanhelsing.studio/posts/extreme-bevy-3
// Add this in the rollback schedule (if a bullet fired by the other player was mis-predicted, this is obviously
// something we’d want to correct!)
pub fn fire_artillery(
    mut commands: Commands,
    inputs: Res<PlayerInputs<RollbackConfig>>,
    mut player_query: Query<(&mut ArtilleryReady, &Transform, &Player)>,
    model_assets: Res<ModelAssets>,
) {
    for (mut artillery_ready, transform, player) in &mut player_query {
        let (input, _) = inputs[player.handle];
        if fire(input) && artillery_ready.0 {
            commands
                .spawn((
                    SceneBundle {
                        scene: model_assets.scene_handles["cannon_ball.glb"].clone(),
                        transform: Transform::from_translation(transform.translation),
                        ..default()
                    },
                    Name::new("Projectile"),
                    Projectile,
                    PhysicsBody,
                    LinearVelocity(Vec3::Y * 18.),
                    LinearAcceleration(Vec3::NEG_Y * 15.),
                    LinearDamping(0.999),
                ))
                .add_rollback();

            artillery_ready.0 = false;
        }
    }
}

pub fn reload_artillery(
    inputs: Res<PlayerInputs<RollbackConfig>>,
    mut player_query: Query<(&mut ArtilleryReady, &Player)>,
) {
    for (mut artillery_ready, player) in &mut player_query {
        let (input, _) = inputs[player.handle];
        if !fire(input) {
            artillery_ready.0 = true;
        }
    }
}
