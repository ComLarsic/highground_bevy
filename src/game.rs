use crate::prelude::*;
use bevy::prelude::*;

/** The state of the game */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Startup,
    Titlescreen,
    Gameplay,
    Menu,
    Cutscene,
}

/// The main plugin that builds the game
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // Init the gamestate
        app.add_state(GameState::Startup);

        // Insert the resources
        app.insert_resource(ClearColor(Color::BLACK));
        app.insert_resource(WindowDescriptor {
            title: "Highground".into(),
            width: 1280.0,
            height: 720.0,
            ..Default::default()
        });

        // Add the plugins
        app.add_plugins(DefaultPlugins);
        app.add_plugin(GameCameraPlugin);
        app.add_plugin(PhysicsPlugin);
        app.add_plugin(PlayerPlugin);

        // Add the systems
        app.add_system_set(SystemSet::on_enter(GameState::Startup).with_system(setup));
    }
}

/// Handles the initial startup for the game
fn setup(
    asset_server: Res<AssetServer>,
    mut state: ResMut<State<GameState>>,
    mut commands: Commands,
) {
    // Set the player
    let player = commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(24.0, 24.0)),
                ..Default::default()
            },
            texture: asset_server.load("sprites/bobert.png"),
            ..Default::default()
        })
        .insert_bundle(PlayerBundle {
            velocity: Vec2::new(0f32, 0f32).into(),
            friction: Vec2::new(12.8f32, 0f32).into(),
            collider: Collider {
                half_extents: Vec2::new(12f32, 12f32),
            },
            gravity_scale: GravityScale(2f32),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::ZERO,
            ..Default::default()
        })
        .id();

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(CameraTarget(player))
        .insert(CameraSpeed(5.0))
        .insert(CameraOffset(Vec2::new(0.0, 24.0)));

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

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::GREEN,
                custom_size: Some(Vec2::new(64f32, 64f32)),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert_bundle(PhysicsBodyBundle {
            body: PhysicsBody::Static,
            velocity: Vec2::new(0f32, 0f32).into(),
            friction: Vec2::new(0f32, 0f32).into(),
            collider: Collider {
                half_extents: Vec2::new(32f32, 32f32),
            },
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(32f32, -256.0f32 + 32f32, 0.0f32),
            ..Default::default()
        });

    // Continue to gameplay
    if *state.current() != GameState::Gameplay {
        state.set(GameState::Gameplay).unwrap();
    }
}
