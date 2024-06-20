use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::Target;

pub struct EnvPlugin;
impl Plugin for EnvPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

// ---

fn startup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut al: ResMut<AmbientLight>
) {
    al.brightness = 100.;
    commands.spawn((
        PbrBundle {
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0., 0., 0.),
                ..default()
            }),
            mesh: meshes.add(Cuboid::from_size(Vec3::new(50.,1.,50.))),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Name::new("Floor"),
        Collider::cuboid(25., 0.5, 25.),
        RigidBody::Fixed,
        Target
    ));
    
    // commands.spawn((
    //     PbrBundle {
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::rgb(0., 0.,1.),
    //             ..default()
    //         }),
    //         mesh: meshes.add(Cuboid::from_size(Vec3::new(50.,50.,50.))),
    //         transform: Transform::from_xyz(500., 500., 500.),
    //         ..default()
    //     },
    //     Name::new("Dummy"),
    //     Collider::cuboid(25., 25., 25.),
    //     RigidBody::Fixed,
    // ));

    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.,
            ..default()
        },
        transform: Transform::from_xyz(100., 100., 100.),
        ..default()
    });
   
    
} 
