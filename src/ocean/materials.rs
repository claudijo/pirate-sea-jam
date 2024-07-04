use std::path::Iter;
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
}

#[derive(ShaderType, Clone, Copy, Reflect, Debug, Default)]
pub struct ImpactPoint {
    pub time: f32,
    pub position: Vec3,
}

// Keep in sync with shader code
const IMPACT_POINTS_CAPACITY: usize = 40;

// Simple circular buffer (that overflows). See https://en.wikipedia.org/wiki/Circular_buffer
#[derive(ShaderType, Clone, Reflect, Debug)]
pub struct ImpactPoints {
    pub write_index: u32,
    pub read_index: u32,
    pub buffer: [ImpactPoint; IMPACT_POINTS_CAPACITY],
}

impl ImpactPoints {
    pub fn new() -> Self {
        Self {
            read_index: 0,
            write_index: 0,
            buffer: [ImpactPoint::default(); IMPACT_POINTS_CAPACITY],
        }
    }
    pub fn put(&mut self, impact_point: ImpactPoint) {
        if (self.write_index + 1) % self.buffer.len() as u32 == self.read_index {
            // Buffer is full. Make room and enable overflow.
            self.read_index = (self.read_index + 1) % self.buffer.len() as u32;
        }
        self.buffer[self.write_index as usize] = impact_point;
        self.write_index = (self.write_index + 1) % self.buffer.len() as u32;
    }

    // fn get(&mut self) -> Some(ImpactPoint) {
    //     if self.read_index == self.write_index {
    //         return None;
    //     }
    //
    //     let result = self.buffer[self.read_index];
    //     self.read_index = (self.read_index + 1) % self.buffer.len() as u32;
    //     result
    // }
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

    #[uniform(103)]
    pub impact_points: ImpactPoints,
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