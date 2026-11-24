use avian3d::prelude::*;
use bevy::prelude::*;

use bevy_enhanced_input::prelude::*;

use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
};

#[derive(Component, Default, Clone)]
struct PlayerChar;

#[derive(Component, Default, Clone)]
pub struct MovementAcceleration(f32);

#[derive(Component, Default, Clone)]
pub struct MovementDampingFactor(f32);

#[derive(Component, Default, Clone)]
pub struct MaxSlopeAngle(f32);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_input_context::<PlayerChar>();
        app.add_observer(player_move);
        app.add_observer(dampen_move);
    }
}

#[derive(InputAction)]
#[action_output(Vec3)]
struct Move;

pub fn player_init() -> impl Scene {
    const PLAYER_SHAPE: Capsule3d = Capsule3d::new(0.5, 1.0);
    bsn! {
        #PC
        Mesh3d(asset_value(PLAYER_SHAPE))
        RigidBody::Dynamic
        Collider::from(PLAYER_SHAPE)
        LockedAxes::ROTATION_LOCKED
        Transform::from_xyz(0.0, 0.5, 0.0)
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min)
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min)
        GravityScale(2.0)
        Children[
            Camera{is_active: true}
            Camera3d
            //FreeCamera
        ]
        PlayerChar
        Actions<PlayerChar> [
            Action<Move>::new()
            ~Scale::splat(1.0)
            ~SwizzleAxis::XZY
            Bindings[
                {preset::cardinal::wasd_keys()}
            ]
        ]
    }
}

fn player_move(
    moving: On<Fire<Move>>,
    mut players: Query<&mut LinearVelocity, With<PlayerChar>>,
    time: Res<Time>,
) {
    let delta_secs = time.delta_secs();
    let &cuber = players.get(moving.context).unwrap();

    for mut linear_velocity in &mut players {
        // Because this is a kinematic body with no friction to stop it
        // Once the velocity is greater than the moving value it flies off

        // This works, the issue now is stopping it from flying off
        // The issue was that I was using the positional data in velocity, so the further cuber went, the faster
        linear_velocity.x += delta_secs * moving.value.x;
        dbg!(&linear_velocity, cuber.x, delta_secs, moving.value.x);
        linear_velocity.z -= delta_secs * moving.value.z;
    }
    //
    // let mut cuber = players.get_mut(moving.context).unwrap();
    // cuber.translation = cuber.translation + moving.value;
}

fn dampen_move(
    _moving: On<Complete<Move>>,
    mut players: Query<&mut LinearVelocity, With<PlayerChar>>,
) {
    for mut linear_velocity in &mut players {
        linear_velocity.x = 0.;
        linear_velocity.z = 0.;
    }
}
