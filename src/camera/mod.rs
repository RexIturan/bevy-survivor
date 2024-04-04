use bevy::prelude::*;

///// Plugin /////
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

///// Systems /////
fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 10.0, 6.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}