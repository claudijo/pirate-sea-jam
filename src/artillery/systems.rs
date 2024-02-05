// use crate::connection::systems::RollbackConfig;
// use crate::inputs::fire;
// use crate::player::components::Player;
// use bevy::prelude::{Commands, Query, Res, Transform};
// use bevy_ggrs::PlayerInputs;

// Check https://johanhelsing.studio/posts/extreme-bevy-3
// Add this in the rollback schedule (if a bullet fired by the other player was mis-predicted, this is obviously
// something we’d want to correct!)
#[allow(dead_code)]
pub fn fire_cannons(// mut commands: Commands,
    // inputs: Res<PlayerInputs<RollbackConfig>>,
    // player_query: Query<(&Transform, &Player)>,
) {
    // Will keep record of fire button being pressed and act on transitions
    // not pressed -> pressed = start aiming
    // pressed -> not pressed = stop aiming, fire cannon

    // Should only be run on fire button state change

    // for (transform, player) in &player_query {
    //     let (input, _) = inputs[player.handle];
    //     if !fire(input) {
    //         // Spawn cannon balls
    //         // Remember to add .add_rollback(); to the entity
    //     }
    // }
}

#[allow(dead_code)]
pub fn move_cannon_ball() {}
