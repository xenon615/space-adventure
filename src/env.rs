use bevy::prelude::*;
use avian3d::prelude::*;


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
            material: materials.add(Color::srgb(0., 0., 0.)),
            mesh: meshes.add(Cuboid::from_size(Vec3::new(100.,1.,100.))),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Name::new("Floor"),
        Collider::cuboid(50., 0.5, 50.),
        RigidBody::Static,
        Target
    ));

    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10000.,
            ..default()
        },
        transform: Transform::from_xyz(100., 100., 100.),
        ..default()
    });

    commands.spawn(DirectionalLightBundle{
        directional_light: DirectionalLight {
            color: Color::srgb(1., 0.65, 0.),
            illuminance: 10000.,
            ..default()
        },
        transform: Transform::from_xyz(-100., 100., -100.),
        ..default()
    });
   
    
} 
