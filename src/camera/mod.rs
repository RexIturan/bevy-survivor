use bevy::prelude::*;

///// Plugin /////
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, zoom_out);
    }
}

///// Systems /////
fn setup(
    mut commands: Commands
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 100.0, 0.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn zoom_out(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>
) {
    for mut camera in &mut query {
        // camera.translation.y += 10. * time.delta().as_secs_f32();
    }
}