use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, ShaderRef, ShaderType,
};

pub type StandardOceanMaterial = ExtendedMaterial<StandardMaterial, OceanMaterialExtension>;

#[derive(ShaderType, Clone, Reflect, Debug, Default)]
pub struct OceanTileSettings {
    pub tile_offset: Vec3,
    pub tile_size: f32,
    pub quad_cell_size: f32,
    pub tier: u32,
    pub time_scale: f32,
    pub waves: [Vec4; 4],
    pub subdivision_count: u32,
}

#[derive(ShaderType, Clone, Reflect, Debug)]
pub struct OceanPosition {
    pub center_offset: Vec3,
}

#[derive(ShaderType, Clone, Reflect, Debug, Default)]
pub struct RollbackTime {
    pub elapsed_seconds: f32,
    pub padding: Vec3, // Needed for wasm, so that type has a size that is a multiple of 16 bytes
}

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct OceanMaterialExtension {
    // We need to ensure that the bindings of the base material and the extension do not conflict,
    // so we start from binding slot 100, leaving slots 0-99 for the base material.
    #[uniform(100)]
    pub settings: OceanTileSettings,

    #[uniform(101)]
    pub position: OceanPosition,

    #[uniform(102)]
    pub rollback_time: RollbackTime,
}

impl MaterialExtension for OceanMaterialExtension {
    fn vertex_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/ocean_material.wgsl".into()
    }

    fn prepass_vertex_shader() -> ShaderRef {
        "shaders/ocean_material_prepass.wgsl".into()
    }
}
