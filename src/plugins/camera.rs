use bevy::asset::LoadState;
use crate::game_state::GameState;
use crate::plugins::ocean::OCEAN_TILE_SIZE;
use crate::plugins::orbiting_camera::OrbitingCamera;
use bevy::core_pipeline::Skybox;
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;
use bevy::render::render_resource::{TextureViewDescriptor, TextureViewDimension};
use bevy::window::{Cursor, CursorGrabMode};

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

pub fn spawn_camera(mut commands: Commands, asset_server: Res<AssetServer>, mut images: ResMut<Assets<Image>>,) {
    let pitch = 30_f32.to_radians();
    let radius = 30. + 15. * pitch;
    let translation = Vec3::new(0.0, pitch.sin() * radius, pitch.cos() * radius);

    let skybox_handle = asset_server.load("skyboxes/basic.png");

    commands.spawn((
        OrbitingCamera {
            pitch,
            radius,
            ..default()
        },
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        // FogSettings {
        //     color: Color::hex("a5cddf").unwrap(),
        //     directional_light_color: Color::rgba(1.0, 0.95, 0.85, 0.5),
        //     directional_light_exponent: 30.0,
        //     falloff: FogFalloff::Linear {
        //         start: OCEAN_TILE_SIZE * 0.25,
        //         end: OCEAN_TILE_SIZE * 2.5,
        //     },
        // },

        Skybox(skybox_handle.clone()),
    ));

    commands.insert_resource(Cubemap {
        is_loaded: false,
        image_handle: skybox_handle,
    });
}

fn configure_skybox_image(
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
    mut cubemap: ResMut<Cubemap>,
) {
    if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        let image = images.get_mut(&cubemap.image_handle).unwrap();

        // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
        // so they appear as one texture. The following code reconfigures the texture as necessary.
        if image.texture_descriptor.array_layer_count() == 1 {
            image.reinterpret_stacked_2d_as_array(image.height() / image.width());
            image.texture_view_descriptor = Some(TextureViewDescriptor {
                dimension: Some(TextureViewDimension::Cube),
                ..default()
            });
        }

        cubemap.is_loaded = true;
    }
}

fn grab_pointer(mut window: Query<&mut Window>) {
    if let Ok(mut window) = window.get_single_mut() {
        window.cursor = Cursor {
            icon: Default::default(),
            visible: false,
            grab_mode: CursorGrabMode::Locked,
            hit_test: true,
        };
    }
}

fn release_pointer(mut window: Query<&mut Window>) {
    if let Ok(mut window) = window.get_single_mut() {
        window.cursor = Cursor::default();
    }
}

fn release_pointer_on_escape(window: Query<&mut Window>, key: Res<Input<KeyCode>>) {
    if key.just_pressed(KeyCode::Escape) {
        release_pointer(window);
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);

        app.add_systems(Update, configure_skybox_image);

        app.add_systems(OnEnter(GameState::InGame), grab_pointer);
        app.add_systems(OnExit(GameState::InGame), release_pointer);

        app.add_systems(
            Update,
            release_pointer_on_escape.run_if(in_state(GameState::InGame)),
        );
    }
}
