use crate::floating_body::systems::float;
use crate::focal_point::resources::FocalPoint;
use crate::game_state::states::GameState;
use crate::ocean::materials::StandardOceanMaterial;
use crate::ocean::resources::Wave;
use crate::ocean::systems::{
    spawn_ocean, sync_ocean_tiles_center_offset, sync_shader_time, update_buoy_water_height,
    update_water_drag,
};
use crate::physics::systems::{update_buoyant_force, update_linear_drag_force};
use bevy::asset::load_internal_asset;
use bevy::prelude::*;
use bevy_ggrs::GgrsSchedule;

mod components;
mod materials;
pub mod resources;
mod systems;

pub const OCEAN_TILE_SIZE: f32 = 160.;

// Needs to be odd
const OCEAN_SECONDARY_TILE_SUBDIVISIONS: u32 = 15;

const OCEAN_PRIMARY_TILE_SUBDIVISIONS: u32 = OCEAN_SECONDARY_TILE_SUBDIVISIONS * 2 + 1;

const WATER_DYNAMICS_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(0x64632a74ee9240ea8097a33da35f3ad5);

const UTILS_HANDLE: Handle<Shader> = Handle::weak_from_u128(0x24c6df2a389f4396aa11f2840f30c5da);

const OCEAN_MATERIAL_BINDINGS: Handle<Shader> =
    Handle::weak_from_u128(0x06a957f34bac4aabad104c64a301c3fb);

// Each Vec4 containing direction x, direction z, steepness, wave_length. Sum of all steepness values must not exceed 1.
const WAVES: [Vec4; 4] = [
    Vec4::new(1., 0., 0.22, 64.),
    Vec4::new(-1., 0.8, 0.2, 48.),
    Vec4::new(1., -1.2, 0.18, 32.),
    Vec4::new(-1., 3., 0.16, 24.),
];

// Following wave will cancel out if sent to gerstener wave function Vec4::new(1., 0., 0., 1.)
// const WAVES: [Vec4; 4] = [
//     Vec4::new(1., 0., 0., 1.),
//     Vec4::new(1., 0., 0., 1.),
//     Vec4::new(1., 0., 0., 1.),
//     Vec4::new(1., 0., 0., 1.),
// ];

const OCEAN_ANIMATION_TIME_SCALE: f32 = 0.6;

pub struct OceanPlugin;

impl Plugin for OceanPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            WATER_DYNAMICS_HANDLE,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/water_dynamics.wgsl"
            ),
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            UTILS_HANDLE,
            concat!(env!("CARGO_MANIFEST_DIR"), "/assets/shaders/utils.wgsl"),
            Shader::from_wgsl
        );

        load_internal_asset!(
            app,
            OCEAN_MATERIAL_BINDINGS,
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/assets/shaders/ocean_material_bindings.wgsl"
            ),
            Shader::from_wgsl
        );

        app.insert_resource(Wave {
            time_scale: OCEAN_ANIMATION_TIME_SCALE,
            sample_count: 4,
            configs: WAVES,
        });

        app.add_plugins(MaterialPlugin::<StandardOceanMaterial>::default());

        app.add_systems(Startup, spawn_ocean);

        app.add_systems(
            Update,
            sync_ocean_tiles_center_offset.run_if(resource_changed::<FocalPoint>()),
        );

        // Animate waves (outside GGRS schedule) when displaying main menu
        app.add_systems(
            Update,
            sync_shader_time.run_if(in_state(GameState::SplashScreen)),
        );

        app.add_systems(GgrsSchedule, sync_shader_time.before(float));

        app.add_systems(
            GgrsSchedule,
            update_buoy_water_height.before(update_buoyant_force),
        );
        app.add_systems(
            GgrsSchedule,
            update_water_drag.before(update_linear_drag_force),
        );
    }
}
