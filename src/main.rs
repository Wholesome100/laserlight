use avian3d::prelude::*;
use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnhancedInputPlugin)
        .add_plugins((
            PhysicsPlugins::default(),
            // Enables debug rendering
            PhysicsDebugPlugin,
        ))
        .add_plugins(FreeCameraPlugin)
        .add_input_context::<PlayerChar>()
        .add_observer(player_move)
        .add_systems(Startup, scene.spawn())
        .run();
}

#[derive(Component, Default, Clone)]
struct PlayerChar;

#[derive(InputAction)]
#[action_output(Vec3)]
struct Move;

/// set up a simple 3D scene
fn scene() -> impl SceneList {
    const STAGE_SHAPE: Cuboid = Cuboid::new(10.0, 1., 10.);
    const PLAYER_SHAPE: Capsule3d = Capsule3d::new(0.5, 1.0);
    bsn_list! {
        #TestStage
        Mesh3d(asset_value(STAGE_SHAPE))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
        RigidBody::Static
        Collider::from(STAGE_SHAPE)
        Transform::from_xyz(0., -2. ,0.)
        --
        #PC
        Mesh3d(asset_value(PLAYER_SHAPE))
        RigidBody::Dynamic
        Collider::from(PLAYER_SHAPE)
        Transform::from_xyz(0.0, 0.5, 0.0)
        Children[
            Camera{is_active: true}
            Camera3d
            FreeCamera
        ]
        PlayerChar
        Actions<PlayerChar> [
            Action<Move>::new()
            ~Scale::splat(0.01)
            ~SwizzleAxis::XZY
            Bindings[
                {preset::cardinal::wasd_keys()}
            ]
        ]
        --
        PointLight {
            shadow_maps_enabled: true,
        }
        Transform::from_xyz(4.0, 8.0, 4.0)
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
        dbg!(&linear_velocity);
        // This works, the issue now is stopping it from flying off
        linear_velocity.x += cuber.x * delta_secs + moving.value.x;
        linear_velocity.z += cuber.z * delta_secs + moving.value.z;
    }
    //
    // let mut cuber = players.get_mut(moving.context).unwrap();
    // cuber.translation = cuber.translation + moving.value;
}
