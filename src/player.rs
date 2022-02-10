use crate::prelude::{Collider, Friction, GameState, GravityScale, PhysicsBody, Velocity};
use bevy::prelude::*;

// The player constants
const PLAYER_WALK_ACCEL: f32 = 12.0;
const PLAYER_RUN_ACCEL: f32 = 17.0;
const PLAYER_JUMP_FORCE: f32 = 135.0;

/// A bundle holding the components for the player
#[derive(Debug, Clone, Bundle)]
pub struct PlayerBundle {
    pub state: PlayerState,
    pub input_state: PlayerInputState,
    pub transform: Transform,
    pub body: PhysicsBody,
    pub velocity: Velocity,
    pub friction: Friction,
    pub gravity_scale: GravityScale,
    pub collider: Collider,
}

/// Represents the player state
#[derive(Debug, Clone, Copy, Component)]
pub enum PlayerState {
    Idle,
    Walking,
    Falling,
    Jumping,
    Attack,
}

/// The controller the player is currently using
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerControllerState {
    Gamepad(usize),
    Keyboard,
}

/// The current player input
#[derive(Debug, Clone, Component)]
pub struct PlayerInputState {
    // The controller the player is currently using
    pub controller: PlayerControllerState,
    // The input on the x-axis
    pub xmove: f32,
    // The flag for if the player is sprinting
    pub is_sprinting: bool,
    // The flag for if the player is jumping
    pub is_jumping: bool,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            state: PlayerState::Idle,
            input_state: Default::default(),
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

impl Default for PlayerInputState {
    fn default() -> Self {
        Self {
            controller: PlayerControllerState::Keyboard,
            xmove: 0.0,
            is_sprinting: false,
            is_jumping: false,
        }
    }
}

/// The plugin for all the player logic
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(
                    update_controller_state
                        .label("update_controller_state")
                        .after("apply_friction"),
                )
                .with_system(
                    update_input_state
                        .label("update_input_state")
                        .after("update_controller_state"),
                )
                .with_system(
                    update_state
                        .label("update_state")
                        .after("update_input_state"),
                )
                .with_system(handle_state.label("handle_state").after("update_state")),
        );
    }
}

/// Update the players state
fn update_state(mut states: Query<(&mut PlayerState, &Velocity, &PlayerInputState)>) {
    // Loop over the player entities
    for (mut state, velocity, input) in states.iter_mut() {
        if velocity.0.y > 0.0 {
            *state = PlayerState::Jumping;
            continue;
        }
        if velocity.0.y < 0.0 {
            *state = PlayerState::Falling;
            continue;
        }
        if input.xmove != 0.0 {
            *state = PlayerState::Walking;
            continue;
        }

        *state = PlayerState::Idle;
    }
}

/// Update the player controller state
fn update_controller_state(
    keys: Res<Input<KeyCode>>,
    gamepads: Res<Input<GamepadButton>>,
    mut states: Query<&mut PlayerInputState>,
) {
    for mut input in states.iter_mut() {
        // Check for gamepad input
        if gamepads.get_just_pressed().len() > 0 {
            input.controller = PlayerControllerState::Gamepad(0);
        }
        // Check for keyboard input
        if keys.get_just_pressed().len() > 0 {
            input.controller = PlayerControllerState::Keyboard;
        }
    }
}

/// Update the players input state
fn update_input_state(
    keys: Res<Input<KeyCode>>,
    gamepad_input: Res<Input<GamepadButton>>,
    gamepad_axes: Res<Axis<GamepadAxis>>,
    mut states: Query<&mut PlayerInputState>,
) {
    for mut input in states.iter_mut() {
        match input.controller {
            PlayerControllerState::Gamepad(id) => {
                // Get the gamepad
                let gamepad = Gamepad(id as usize);
                // Check if the player is sprinting
                input.is_sprinting =
                    gamepad_input.pressed(GamepadButton(gamepad, GamepadButtonType::West));
                // Check if the player is trying to jump
                input.is_jumping =
                    gamepad_input.just_pressed(GamepadButton(gamepad, GamepadButtonType::South));
                // Update the horizontal input
                input.xmove = gamepad_axes
                    .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
                    .unwrap();
            }
            PlayerControllerState::Keyboard => {
                // Check if the player is sprinting
                input.is_sprinting = keys.pressed(KeyCode::LShift);
                // Check if the player is trying to jump
                input.is_jumping = keys.just_pressed(KeyCode::Space);
                // Update the horizontal input
                input.xmove = -(keys.pressed(KeyCode::A) as i32 as f32)
                    + keys.pressed(KeyCode::D) as i32 as f32;
            }
        };
    }
}

/// Handle the players state
fn handle_state(
    time: Res<Time>,
    mut players: Query<(&mut Velocity, &PlayerState, &PlayerInputState)>,
) {
    for (mut velocity, state, input) in players.iter_mut() {
        match state {
            PlayerState::Idle => {
                if input.is_jumping && velocity.0.y == 0.0 {
                    velocity.0.y += PLAYER_JUMP_FORCE;
                }
            }
            PlayerState::Walking => {
                if input.is_jumping && velocity.0.y == 0.0 {
                    velocity.0.y += PLAYER_JUMP_FORCE;
                }

                velocity.0.x += input.xmove
                    * if input.is_sprinting {
                        PLAYER_RUN_ACCEL
                    } else {
                        PLAYER_WALK_ACCEL
                    }
                    * time.delta_seconds()
                    * 100.0;
            }
            PlayerState::Falling => {
                velocity.0.x += input.xmove
                    * if input.is_sprinting {
                        PLAYER_RUN_ACCEL
                    } else {
                        PLAYER_WALK_ACCEL
                    }
                    * time.delta_seconds()
                    * 100.0;
            }
            PlayerState::Jumping => {
                velocity.0.x += input.xmove
                    * if input.is_sprinting {
                        PLAYER_RUN_ACCEL
                    } else {
                        PLAYER_WALK_ACCEL
                    }
                    * time.delta_seconds()
                    * 100.0;
            }
            PlayerState::Attack => {}
        };
    }
}
