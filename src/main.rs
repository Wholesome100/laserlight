//! Laserlight is a First Person Shooter
//!
//! It is going to be a very epic game
//!
//! I like pancakes

use avian3d::prelude::*;
use bevy::{camera_controller::free_camera::FreeCameraPlugin, prelude::*};
use bevy_enhanced_input::prelude::*;

use crate::character::player::player_init as make_player;

pub mod character;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EnhancedInputPlugin)
        .add_plugins((
            PhysicsPlugins::default(),
            // Enables debug rendering
            PhysicsDebugPlugin,
        ))
        .add_plugins((FreeCameraPlugin, character::player::PlayerPlugin))
        .add_systems(Startup, scene.spawn())
        .run();
}

/// set up a simple 3D scene
fn scene() -> impl SceneList {
    const STAGE_SHAPE: Cuboid = Cuboid::new(100., 1., 100.);
    bsn_list! {
        #TestStage
        Mesh3d(asset_value(STAGE_SHAPE))
        MeshMaterial3d::<StandardMaterial>(asset_value(Color::WHITE))
        RigidBody::Static
        Collider::from(STAGE_SHAPE)
        Transform::from_xyz(0., -2. ,0.)
        --
        @make_player()
        --
        PointLight {
            shadow_maps_enabled: true,
        }
        Transform::from_xyz(4.0, 8.0, 4.0)
    }
}
