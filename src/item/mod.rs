use rand::{rngs::StdRng, Rng, SeedableRng};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::prelude::ColliderMassProperties::Mass;
use crate::spawner::PositionIterator;

///// PlugIn /////
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup);
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
    let mesh_handle = meshes.add(Cuboid::new(0.5, 0.5, 0.5));
    let mat_handle = materials.add(Color::BLUE);

    let (width, height) = (100, 200);
    let mut position_iter = PositionIterator::new(width, height);

    let count = width * height;

    let pos_offset = Vec2::new(width as f32 / -2., height as f32 / -2.);

    for _ in 0..count {
        let plane_pos = position_iter.next().unwrap() + pos_offset;

        let position = Vec3::new(
            plane_pos.x,
            0.25,
            plane_pos.y,
        );

        let velocity = Vec3::new(
            rng.gen_range(vel_range.clone()),
            0.,
            rng.gen_range(vel_range.clone()),
        );

        commands.spawn(RigidBody::Fixed)
            .insert(LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED)
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
            .insert(Collider::ball(0.5))
            .insert(Sensor);
    }
}