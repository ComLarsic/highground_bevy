use bevy::prelude::*;

/** Represents the physics body */
#[derive(Debug, Clone, Component, PartialEq, Eq)]
pub enum PhysicsBody {
    Dynamic,
    Static,
}

/** Represents a physics body's velocity */
#[derive(Debug, Clone, Component)]
pub struct Velocity(pub Vec2);

/** Represents a phsyics body's friction */
#[derive(Debug, Clone, Component)]
pub struct Friction(pub Vec2);

/** Represents the gravity scale */
#[derive(Debug, Clone, Component)]
pub struct GravityScale(pub f32);

impl From<Vec2> for Velocity {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

impl From<Vec2> for Friction {
    fn from(v: Vec2) -> Self {
        Self(v)
    }
}

/** Represents an AABB physics collider */
#[derive(Debug, Clone, Component)]
pub struct Collider {
    pub half_extents: Vec2,
}

/** A bundle for a physics bundle */
#[derive(Debug, Clone, Bundle)]
pub struct PhysicsBodyBundle {
    pub transform: Transform,
    pub body: PhysicsBody,
    pub velocity: Velocity,
    pub friction: Friction,
    pub gravity_scale: GravityScale,
    pub collider: Collider,
}

impl Default for PhysicsBodyBundle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            body: PhysicsBody::Dynamic,
            velocity: Vec2::ZERO.into(),
            friction: Vec2::ZERO.into(),
            collider: Collider {
                half_extents: Vec2::ZERO,
            },
            gravity_scale: GravityScale(1.0),
        }
    }
}

/** The config for the physics */
#[derive(Debug)]
pub struct PhysicsConfig {
    pub enabled: bool,
    pub gravity: Vec2,
}

impl Default for PhysicsConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            gravity: Vec2::new(0f32, -98.1f32),
        }
    }
}

/** The games physics plugin */
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhysicsConfig>();

        app.add_system(apply_gravity.label("apply_gravity"));
        app.add_system(
            handle_collisions
                .label("handle_collision")
                .label("apply_gravity"),
        );
        app.add_system(
            apply_velocity
                .label("apply_velocity")
                .after("handle_collision"),
        );
        app.add_system(
            apply_friction
                .label("apply_friction")
                .after("apply_velocity"),
        );
    }
}

/** Handle collisions between the bodies */
#[rustfmt::skip]
fn handle_collisions(time: Res<Time>, mut bodies: Query<(&mut Velocity, &Collider, &Transform, &PhysicsBody)>) {
    let mut iter = bodies.iter_combinations_mut();
    while let Some([
        (mut velocity_a, collider_a, transform_a, _), 
        (mut velocity_b, collider_b, transform_b, _)
    ]) = iter.fetch_next()
    {   
        // Calulate the next positions for body a
        let next_x = transform_a.translation.x + velocity_a.0.x * time.delta_seconds();
        let next_y = transform_a.translation.y + velocity_a.0.y * time.delta_seconds();
        
        // Check the collisions between the different axises
        if !((next_x - transform_b.translation.x).abs() > collider_a.half_extents.x + collider_b.half_extents.x) &&
            !((transform_a.translation.y - transform_b.translation.y).abs() > collider_a.half_extents.y + collider_b.half_extents.y) {
            velocity_a.0.x = 0.0;
        }
        if !((transform_a.translation.x - transform_b.translation.x).abs() > collider_a.half_extents.x + collider_b.half_extents.x) &&
            !((next_y - transform_b.translation.y).abs() > collider_a.half_extents.y + collider_b.half_extents.y) {
            velocity_a.0.y = 0.0;
        }

        // Calulate the next positions for body b
        let next_x = transform_b.translation.x + velocity_b.0.x * time.delta_seconds();
        let next_y = transform_b.translation.y + velocity_b.0.y * time.delta_seconds();
        
        // Check the collisions between the different axises
        if !((next_x - transform_a.translation.x).abs() > collider_b.half_extents.x + collider_a.half_extents.x) &&
            !((transform_b.translation.y - transform_a.translation.y).abs() > collider_b.half_extents.y + collider_a.half_extents.y) {
            velocity_b.0.x = 0.0;
        }
        if !((transform_b.translation.x - transform_a.translation.x).abs() > collider_b.half_extents.x + collider_a.half_extents.x) &&
            !((next_y - transform_a.translation.y).abs() > collider_b.half_extents.y + collider_a.half_extents.y) {
            velocity_b.0.y = 0.0;
        }
    }
}

/** Apply the gravity to the body */
fn apply_gravity(
    time: Res<Time>,
    physics_conf: Res<PhysicsConfig>,
    mut bodies: Query<(&mut Velocity, &PhysicsBody, &GravityScale)>,
) {
    for (mut velocity, body, scale) in bodies.iter_mut() {
        if *body == PhysicsBody::Dynamic {
            velocity.0 += physics_conf.gravity * scale.0 * time.delta_seconds();
        }
    }
}

/** Applies the velocity to the bodies */
fn apply_velocity(time: Res<Time>, mut bodies: Query<(&mut Transform, &PhysicsBody, &Velocity)>) {
    for (mut transform, body, velocity) in bodies.iter_mut() {
        if *body == PhysicsBody::Dynamic {
            transform.translation.x += velocity.0.x * time.delta_seconds();
            transform.translation.y += velocity.0.y * time.delta_seconds();
        }
    }
}

/** Applies the friction to the bodies*/
fn apply_friction(time: Res<Time>, mut bodies: Query<(&mut Velocity, &PhysicsBody, &Friction)>) {
    for (mut velocity, body, friction) in bodies.iter_mut() {
        if *body == PhysicsBody::Dynamic {
            velocity.0.x += (friction.0.x * time.delta_seconds()) * (0f32 - velocity.0.x);
            velocity.0.y += (friction.0.y * time.delta_seconds()) * (0f32 - velocity.0.y);
        }
    }
}
