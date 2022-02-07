//! A story-driven 2d platformer with rpg-elements, inspired by super paper mario
use bevy::prelude::*;
use libhighground::prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW,
                custom_size: Some(Vec2::new(64f32, 64f32)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(PlayerBundle {
            velocity: Vec2::new(0f32, 0f32).into(),
            friction: Vec2::new(0f32, 0f32).into(),
            collider: Collider {
                half_extents: Vec2::new(32f32, 32f32),
            },
            gravity_scale: GravityScale(2f32),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::ZERO,
            ..Default::default()
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(512f32, 64f32)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(PhysicsBodyBundle {
            body: PhysicsBody::Static,
            velocity: Vec2::new(0f32, 0f32).into(),
            friction: Vec2::new(0f32, 0f32).into(),
            collider: Collider {
                half_extents: Vec2::new(256f32, 32f32),
            },
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(-64f32, -256.0f32, 0.0f32),
            ..Default::default()
        });
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Highground".into(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup)
        .run();
}
