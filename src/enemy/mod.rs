use rand::{rngs::StdRng, Rng, SeedableRng};
use bevy::prelude::*;

const NUM_ENEMIES: usize = 10000;

///// PlugIn /////
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, move_enemy);
    }
}


///// Systems /////
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let pos_range = -1.0..1.0;
    let vel_range = -0.5..0.5;

    let mut rng = StdRng::seed_from_u64(19878367467713);
    let mesh_handle = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let mat_handle = materials.add(Color::CRIMSON);

    commands.spawn(EnemyBundle {
        visuals: PbrBundle {
            mesh: mesh_handle.clone(),
            material: mat_handle.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        velocity: Velocity( Vec3::new(
            rng.gen_range(vel_range.clone()),
            0.,
            rng.gen_range(vel_range.clone()),
        )),
        ..default()
    });

    for _ in 0..NUM_ENEMIES {
        let position = Vec3::new(
            rng.gen_range(pos_range.clone()),
            0.5,
            rng.gen_range(pos_range.clone()),
        )
        .normalize()
            * rng.gen_range(0.2f32..1.0).cbrt();

        commands.spawn(EnemyBundle {
            visuals: PbrBundle {
                mesh: mesh_handle.clone(),
                material: mat_handle.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            velocity: Velocity( Vec3::new(
                rng.gen_range(vel_range.clone()),
                0.,
                rng.gen_range(vel_range.clone()),
            )),
            ..default()
        });
    }
}

fn move_enemy(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Enemy>>
) {
    for (mut transform, velocity) in &mut query {
        transform.translation += velocity.0 * time.delta().as_secs_f32();
    }
}

///// Components /////
#[derive(Component)]
struct Velocity(Vec3);

#[derive(Component)]
struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    visuals: PbrBundle,
    velocity: Velocity
}

const INITIAL_ENEMY_DIRECTION: Vec3 = Vec3::new(0.1, 0.0, 0.0);

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            visuals: PbrBundle {
                ..PbrBundle::default()
            },
            enemy: Enemy,
            velocity: Velocity(INITIAL_ENEMY_DIRECTION)
        }
    }
}