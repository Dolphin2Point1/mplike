use bevy_backroll::BackrollPlugin;
use bevy::prelude::*;

use bevy::input::{gamepad::gamepad_event_system, system::exit_on_esc_system};
use bevy::pbr::SpecializedMaterial;
use bevy::render::render_asset::RenderAsset;
use crate::board::space_shader::CircleMaterial;

pub mod input;
pub mod board;


fn main() {
    App::new()
        // add plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CircleMaterial>::default())
        // resources
        .insert_resource(WindowDescriptor {
            title: "aaa".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        .insert_resource(Msaa {
            samples: 4
        })
        // startup systems
        .add_startup_system(setup)
        // AAAAAAAA
        .add_system(exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CircleMaterial>>,
) {
    commands.spawn_bundle(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 2.0 })),
        material: materials.add(CircleMaterial {
            circle_color: Color::Rgba {
                red: 0.0,
                green: 0.0,
                blue: 1.0,
                alpha: 1.0
            },
            outline_color: Color::Rgba {
                red: 1.0,
                green: 0.843137254902,
                blue: 0.0,
                alpha: 1.0
            },
            outer_color: Color::Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 0.0
            }
        }),
        global_transform: GlobalTransform {
            translation: Vec3::new(0.0,0.0,0.1),
            scale: Vec3::new(1.0,1.0,1.0),
            ..Default::default()
        },
        ..Default::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 10.0, 1.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
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
