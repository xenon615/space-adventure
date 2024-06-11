use bevy::{prelude::*, transform::commands};
use bevy_xpbd_3d::{components::{ExternalForce, Mass, RigidBody}, plugins::collision::Collider, *};
pub struct DummyPlugin;
impl  Plugin for DummyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn, 
            // test
        ).chain());
    }
}

#[derive(Component)]
pub struct Dummy;

fn spawn (
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(2., 1.,2.)),
            material: materials.add(Color::RED),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Collider::cuboid(2., 1.,2.),
        Dummy,
        RigidBody::Dynamic,
        ExternalForce::new(Vec3::Y * 9.81 * 4.).with_persistence(true)
    ));
}


// fn test(
//     d_q: Query<Mass>
// ) {

// }