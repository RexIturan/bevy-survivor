use bevy::prelude::*;

///// Plugin /////
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

///// Systems /////
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //floor
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Plane3d::default().mesh().size(50.0, 50.0)),
    //     material: materials.add(Color::rgb(0.3, 0.3, 0.3)),
    //     ..default()
    // });

    //todo idk in what direction the light is looking
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(
            Quat::from_euler(EulerRot::XYZ, -45.0, 45.0,0.0)
        ),
        ..default()
    });
}