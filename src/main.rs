mod tutorial;
mod enemy;
mod camera;
mod scene;

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)


        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)

        // 3rd party plugins
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)

        // own plugins
        .add_plugins(tutorial::HelloPlugin)
        .add_plugins(enemy::EnemyPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(scene::ScenePlugin)

        .run();
}

fn setup(mut commands: Commands) {
    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiCompleteBundle::default());
}