mod tutorial;
mod enemy;
mod camera;
mod scene;
mod mesh_instancing;
mod spawner;

use bevy::prelude::*;
use bevy::diagnostic::{
    FrameTimeDiagnosticsPlugin,
    EntityCountDiagnosticsPlugin,
    SystemInformationDiagnosticsPlugin
};

use iyes_perf_ui::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)


        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(EntityCountDiagnosticsPlugin)
        .add_plugins(SystemInformationDiagnosticsPlugin)

        // 3rd party plugins
        .add_plugins(PerfUiPlugin)
        .add_systems(Startup, setup)

        // own plugins
        .add_plugins(scene::ScenePlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(enemy::EnemyPlugin)

        .run();
}

fn setup(mut commands: Commands) {
    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiCompleteBundle::default());
}