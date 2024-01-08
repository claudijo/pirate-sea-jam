use crate::components::ship::PlayerShip;
use crate::resources::wave::Wave;
use crate::utils::sphere;
use crate::utils::water_dynamics;
use crate::utils::water_dynamics::SPHERE_DRAG_COEFFICIENT;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

pub const BALSA_DENSITY: f32 = 0.16;
pub const CORK_DENSITY: f32 = 0.25;
pub const PINE_DENSITY: f32 = 0.45;
pub const OAK_DENSITY: f32 = 0.75;

#[derive(Component)]
pub struct Buoy;

#[derive(Bundle)]
pub struct BuoyBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub collider: Collider,
    pub collider_density: ColliderDensity,
    pub buoy: Buoy,
    pub rigid_body: RigidBody,
    pub linear_damping: LinearDamping,
    pub angular_damping: AngularDamping,
    pub collision_layers: CollisionLayers,
    pub external_force: ExternalForce,
}

impl Default for BuoyBundle {
    fn default() -> Self {
        Self {
            transform: Transform::default(),
            global_transform: GlobalTransform::default(),
            collider: Collider::ball(1.),
            collider_density: ColliderDensity(BALSA_DENSITY),
            rigid_body: RigidBody::Dynamic,
            linear_damping: LinearDamping::default(),
            angular_damping: AngularDamping::default(),
            collision_layers: CollisionLayers::none(),
            external_force: ExternalForce::default(),
            buoy: Buoy,
        }
    }
}

impl BuoyBundle {
    pub fn from_transform(transform: Transform) -> Self {
        Self {
            transform,
            ..default()
        }
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.collider = Collider::ball(radius);
        self
    }
}

fn spawn_buoy(
    mut commands: Commands,
) {
    commands.spawn((
        BuoyBundle::default(),
    ));
}

fn float(
    ship_query: Query<&Transform, With<PlayerShip>>,
    mut buoy_query: Query<
        (
            &GlobalTransform,
            &LinearVelocity,
            &Collider,
            &mut ExternalForce,
            &mut LinearDamping,
            &mut AngularDamping,
        ),
        With<Buoy>,
    >,
    wave: Res<Wave>,
    time: Res<Time>,
) {
    let elapsed_time = time.elapsed().as_secs_f32();

    let ship_translation = if let Ok(ship_transform) = ship_query.get_single() {
        ship_transform.translation
    } else {
        Vec3::ZERO
    };

    for (
        buoy_global_transform,
        linear_velocity,
        collider,
        mut external_force,
        mut linear_damping,
        mut angular_damping,
    ) in &mut buoy_query
    {
        let translation = buoy_global_transform.translation();
        let water_height = wave.surface_height(translation - ship_translation, elapsed_time);
        let radius = collider.shape().as_ball().unwrap().radius;
        let displaced_liquid_volume =
            sphere::displaced_liquid_volume(radius, translation.y, water_height);

        let buoyant_force = water_dynamics::buoyant_force(displaced_liquid_volume);
        let submerged = (translation.y - water_height) - radius;

        let damping_coefficient;
        if submerged >= 0. {
            // Not submerged
            damping_coefficient = 0.;
        } else if submerged < -radius {
            // At least half submerged
            damping_coefficient = water_dynamics::damping(
                linear_velocity.y,
                sphere::cross_section_area(radius),
                SPHERE_DRAG_COEFFICIENT,
            );
        } else {
            // Less than half submerged
            damping_coefficient = water_dynamics::damping(
                linear_velocity.y,
                sphere::off_center_cross_section_area(radius, radius + submerged),
                SPHERE_DRAG_COEFFICIENT,
            );
        }

        external_force.set_force(buoyant_force);
        linear_damping.0 = damping_coefficient;
        angular_damping.0 = damping_coefficient;
    }
}

fn ignore_collisions(
    mut collisions: ResMut<Collisions>,
    query: Query<(), With<Buoy>>,
) {
    // Remove collisions where one of the colliders has an `Invulnerable` component.
    // In a real project, this could be done more efficiently with collision layers.
    collisions.retain(|contacts| {
        !query.contains(contacts.entity1) && !query.contains(contacts.entity2)
    });
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
        // app.add_systems(Update, keep_at_water_level);
        app.add_systems(Update, float);
        //app.add_systems(PostProcessCollisions, ignore_collisions);
    }
}
