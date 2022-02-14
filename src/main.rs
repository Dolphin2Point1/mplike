use bevy::prelude::*;
use bevy::prelude::shape::Plane;
use bevy_backroll::BackrollPlugin;

use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        // resources
        .insert_resource(WindowDescriptor {
            title: "MP-Like Game".to_string(),
            width: 1280.,
            height: 720.,
            ..Default::default()
        })
        // add plugins
        .add_plugins(DefaultPlugins)

        .add_plugin(BackrollPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        // startup systems
        .add_startup_system(setup)
        // AAAAAAAA
        .add_system(bevy::input::system::exit_on_esc_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(Plane { size: 8.0 })),
        material: materials.add(Color::rgb(1., 0.9, 0.9).into()),
        transform: Transform::from_translation(Vec3::new(4., 0., 4.)),
        ..Default::default()
    });
    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(Mat4::from_rotation_translation(
            Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
            Vec3::new(-7.0, 20.0, 4.0),
        )),
        ..Default::default()
    });
    // Light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}