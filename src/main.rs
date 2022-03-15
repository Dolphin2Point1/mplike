use bevy_backroll::BackrollPlugin;
use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
use bevy::input::{gamepad::gamepad_event_system, system::exit_on_esc_system};
use crate::board::space::CircleMaterial;

pub mod input;
pub mod board;


fn main() {
    
    App::new()
        // resources
        .insert_resource(WindowDescriptor {
            title: "MP-Like Game".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        // .insert_resource(CircleMaterial {
        //     inner_color: Color::Rgba {
        //         red: 1.0,
        //         green: 0.843137254902,
        //         blue: 0.0,
        //         alpha: 1.0
        //     },
        //     outer_color: Color::Rgba {
        //         red: 0.0,
        //         green: 0.0,
        //         blue: 1.0,
        //         alpha: 1.0
        //     }
        // })
        // add plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(BackrollPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        // startup systems
        .add_startup_system(setup)
        // AAAAAAAA
        .add_system(exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn_scene(asset_server.load("boards/testmap.gltf#Scene0"));
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(21.2536, 14.1089, -18.3229).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });
}
