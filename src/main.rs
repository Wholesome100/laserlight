use avian3d::prelude::*;
use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            PhysicsPlugins::default(),
            // Enables debug rendering
            PhysicsDebugPlugin,
        ))
        .add_plugins(FreeCameraPlugin)
        .add_systems(Startup, scene.spawn())
        .run();
}

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
        --
        PointLight {
            shadow_maps_enabled: true,
        }
        Transform::from_xyz(4.0, 8.0, 4.0)
    }
}
