use rand::{rngs::StdRng, Rng, SeedableRng};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::prelude::ColliderMassProperties::Mass;
use crate::spawner::PositionIterator;

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

    let (width, height) = (100, 20);
    let mut position_iter = PositionIterator::new(width, height);

    let count = width * height;

    let pos_offset = Vec2::new(width as f32 / -2., height as f32 / -2.);

    for _ in 0..count {
        let plane_pos = position_iter.next().unwrap() + pos_offset;

        let position = Vec3::new(
            plane_pos.x,
            0.5,
            plane_pos.y,
        );

        let velocity = Vec3::new(
            rng.gen_range(vel_range.clone()),
            0.,
            rng.gen_range(vel_range.clone()),
        );

        commands.spawn(RigidBody::Dynamic)
            .insert(LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED)
            .insert(Velocity {
                linvel: velocity,
                angvel: Vec3::ZERO,
            })
            .insert(ExternalForce {
                force: velocity,
                torque: Vec3::ZERO
            })
            .insert(TransformBundle::from(Transform::from_translation(position)))
            .insert(PbrBundle {
                mesh: mesh_handle.clone(),
                material: mat_handle.clone(),
                transform: Transform::from_translation(position),
                ..default()
            })
            .insert(Mass(1.))
            .insert(Friction::new(0.))
            // .insert(Collider::cuboid(0.5, 0.5, 0.5))
            .insert(Collider::ball(0.5));

        // commands.spawn(EnemyBundle {
        //     visuals: PbrBundle {
        //         mesh: mesh_handle.clone(),
        //         material: mat_handle.clone(),
        //         transform: Transform::from_translation(position),
        //         ..default()
        //     },
        //     velocity: Velocity {
        //         linvel: Vec3::new(
        //             rng.gen_range(vel_range.clone()),
        //             0.,
        //             rng.gen_range(vel_range.clone()),
        //         ),
        //         ..default()
        //     },
        //     rigid_body: RigidBody::Dynamic,
        //     ..default()
        // });
    }

    // for _ in 0..NUM_ENEMIES {
    //     let position = Vec3::new(
    //         rng.gen_range(pos_range.clone()),
    //         0.5,
    //         rng.gen_range(pos_range.clone()),
    //     )
    //     .normalize()
    //         * rng.gen_range(0.2f32..1.0).cbrt();
    //
    //     commands.spawn(EnemyBundle {
    //         visuals: PbrBundle {
    //             mesh: mesh_handle.clone(),
    //             material: mat_handle.clone(),
    //             transform: Transform::from_translation(position),
    //             ..default()
    //         },
    //         velocity: Velocity( Vec3::new(
    //             rng.gen_range(vel_range.clone()),
    //             0.,
    //             rng.gen_range(vel_range.clone()),
    //         )),
    //         ..default()
    //     });
    // }
}

fn move_enemy(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Enemy>>
) {
    for (mut transform, velocity) in &mut query {
        // transform.translation += velocity.0 * time.delta().as_secs_f32();
    }
}

///// Components /////
// #[derive(Component)]
// struct Velocity(Vec3);

#[derive(Component)]
struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    visuals: PbrBundle,
    velocity: Velocity,
    collider: Collider,
    rigid_body: RigidBody
}

const INITIAL_ENEMY_DIRECTION: Vec3 = Vec3::new(0.1, 0.0, 0.0);

impl Default for EnemyBundle {
    fn default() -> Self {
        Self {
            visuals: PbrBundle {
                ..PbrBundle::default()
            },
            enemy: Enemy,
            velocity: Velocity {
                linvel: INITIAL_ENEMY_DIRECTION,
                angvel: Vec3::new(0., 0., 0.),
            },
            collider: Collider::cuboid(50., 50., 50.),
            rigid_body: RigidBody::Dynamic
        }
    }
}