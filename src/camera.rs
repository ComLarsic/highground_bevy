use bevy::prelude::*;

/// The target for the camera to follow
#[derive(Debug, Clone, Component)]
pub struct CameraTarget(pub Entity);

/// The camera offset
#[derive(Debug, Default, Clone, Component)]
pub struct CameraOffset(pub Vec2);

/// The lerping speed for the camera
#[derive(Debug, Default, Clone, Component)]
pub struct CameraSpeed(pub f32);

/// The plugin that handles the game camera
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_scaling);
        app.add_system(update_position);
    }
}

/// Update the camera scaling
fn update_scaling(
    windows: Res<Windows>,
    mut cameras: Query<(&Camera, &mut OrthographicProjection)>,
) {
    for (camera, mut projection) in cameras.iter_mut() {
        let window = windows.get(camera.window).unwrap();
        projection.scale = 320.0 / window.width();
    }
}

/// Move towards the lerping position
fn update_position(
    time: Res<Time>,
    transform_entities: Query<(Entity, &Transform, Without<Camera>)>,
    mut cameras: Query<(
        &mut Transform,
        &CameraTarget,
        &CameraSpeed,
        Option<&CameraOffset>,
        With<Camera>,
    )>,
) {
    for (mut transform, target, speed, offset, _) in cameras.iter_mut() {
        let (_, target, _) = transform_entities.get(target.0).unwrap();

        if let Some(offset) = offset {
            transform.translation.x += (speed.0 * time.delta_seconds())
                * (target.translation.x + offset.0.x - transform.translation.x);
            transform.translation.y += (speed.0 * time.delta_seconds())
                * (target.translation.y + offset.0.y - transform.translation.y);
        } else {
            transform.translation.x +=
                (speed.0 * time.delta_seconds()) * (target.translation.x - transform.translation.x);
            transform.translation.y +=
                (speed.0 * time.delta_seconds()) * (target.translation.y - transform.translation.y);
        }
    }
}
