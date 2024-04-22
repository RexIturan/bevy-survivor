use bevy::{
    prelude::*,
    render::{
        extract_component::{ExtractComponent},
        view::{NoFrustumCulling},
    },
};
use bevy::asset::Assets;
use bevy::math::Vec3;
use bevy::prelude::{Commands, Mesh, ResMut};
use bevy_rapier3d::dynamics::{ExternalForce, LockedAxes, RigidBody};
use bevy_rapier3d::geometry::ColliderMassProperties::Mass;
use bevy_rapier3d::geometry::{Collider, Friction};
use bevy_rapier3d::prelude::Velocity;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::mesh_instancing::*;

pub struct StressTestPlugin;

impl Plugin for StressTestPlugin {

    fn build(&self, app: &mut App) {
        app
            .add_systems(Last, (update_instance_item, collect_instance_data).chain());

        app.add_systems(Startup, demo_setup);
    }
}


fn demo_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let (width, height) = (100, 100);
    let offset = Vec3::new(width as f32 / 2., 0., height as f32 / 2.);

    let mut rng = StdRng::seed_from_u64(19878367467713);
    let vel_range = -0.5..0.5;

    for y in 0..height {
        for x in 0..width {
            let velocity = Vec3::new(
                rng.gen_range(vel_range.clone()),
                0.,
                rng.gen_range(vel_range.clone()),
            );

            let posX = x as f32 / 10.;
            let posY = y as f32 / 10.;

            let position = Vec3::new(posX * 10., 0.0, posY * 10.0) - offset;

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
                .insert(Mass(1.))
                .insert(Friction::new(1.))
                // .insert(Collider::cuboid(0.5, 0.5, 0.5))
                .insert(Collider::ball(0.5))
                .insert(InstanceMaterialItem {
                    id: 0,
                    data: InstanceData {
                        position,
                        scale: 1.0,
                        color: Color::hsla(posX * 360., posY, 0.5, 1.0).as_rgba_f32(),
                    }
                });
        }
    }

    commands.spawn((
        meshes.add(Cuboid::new(0.5, 0.5, 0.5)),
        SpatialBundle::INHERITED_IDENTITY,
        InstanceMaterialData {
            id: 0,
            data: (1..=10)
                .flat_map(|x| (1..=10).map(move |y| (x as f32 / 10.0, y as f32 / 10.0)))
                .map(|(x, y)| InstanceData {
                    position: Vec3::new(x * 10.0 - 5.0, 0.0, y * 10.0 - 5.0),
                    scale: 1.0,
                    color: Color::hsla(x * 360., y, 0.5, 1.0).as_rgba_f32(),
                })
                .collect(),

        },
        // NOTE: Frustum culling is done based on the Aabb of the Mesh and the GlobalTransform.
        // As the cube is at the origin, if its Aabb moves outside the view frustum, all the
        // instanced cubes will be culled.
        // The InstanceMaterialData contains the 'GlobalTransform' information for this custom
        // instancing, and that is not taken into account with the built-in frustum culling.
        // We must disable the built-in frustum culling by adding the `NoFrustumCulling` marker
        // component to avoid incorrect culling.
        NoFrustumCulling,
    ));
}

#[derive(Component)]
struct InstanceMaterialItem {
    pub id: u32,
    pub data: InstanceData
}

fn update_instance_item (
    mut query: Query<(&mut InstanceMaterialItem, &Transform)>,
) {
    for (mut instance_item, transform) in &mut query {
        instance_item.data.position = transform.translation;
    }
}

fn collect_instance_data(
    collect_query: Query<&InstanceMaterialItem>,
    mut write_query: Query<&mut crate::mesh_instancing::InstanceMaterialData>
) {
    for mut instance_data in &mut write_query {
        instance_data.data.clear();
        let mut index = 0;
        for item_data in &collect_query {
            if item_data.id == instance_data.id {
                instance_data.data.insert(index, item_data.data.clone());
                index += 1;
            }
        }
    }
}