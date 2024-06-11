use bevy:: prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy_xpbd_3d::{components::{CoefficientCombine, ColliderDensity, LinearDamping,ExternalForce, GravityScale, Mass, Restitution, RigidBody}, plugins::collision::Collider, resources::Gravity};
pub struct _DronePlugin;
impl Plugin for _DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, control);
        // app.add_systems(Update, children);
        app.add_systems(Update, set_nozzles);
        app.add_systems(Update, set_forces);

        app.register_type::<Nozzle>();
        app.register_type::<SLeft>();
        app.register_type::<SRight>();
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Nozzle;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct SLeft;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct SRight;

#[derive(Component)]
pub struct Drone;

#[derive(Component)]
pub struct Nozzles(pub Vec<Entity>);

#[derive(Component)]
pub struct ForceValue(pub f32);


fn spawn(
    mut commands: Commands,
    asset: ResMut<AssetServer>
) {
    commands.spawn((
        SceneBundle {
            scene: asset.load("models/drone.glb#Scene0"),
            transform: Transform::from_xyz(0., 0., 0.).looking_at(Vec3::Z, Vec3::Y),
            ..default()
        },
        Name::new("Drone"),
        RigidBody::Dynamic,
        Collider::cuboid(6., 0.5, 2.),
        // Collider::cuboid(1., 1., 1.),
        // ColliderDensity(20.),
        GravityScale(1.),
        ExternalForce::new(Vec3::ZERO).with_persistence(false),
        // ExternalForce::default(),
        ForceValue(0.),
        LinearDamping(0.8),
        Drone
    ));
} 

fn control(
    mut q: Query<(&mut Transform, &GlobalTransform), With<Nozzle>>,
    mut drone_q: Query<(&mut ForceValue, &Mass), With<Drone>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut scroll_evt: EventReader<MouseWheel>,
    mut gizmos: Gizmos,
) {
    
    for (mut n, ng) in q.iter_mut() {
        // if keys.pressed(KeyCode::ArrowRight) {
        //     ct.rotate_y(-0.05);
        // }
        // if keys.pressed(KeyCode::ArrowLeft) {
        //     ct.rotate_y(0.05);
        // }
        gizmos.ray(ng.translation(), ng.down() * 10., Color::RED);

        if keys.pressed(KeyCode::ArrowUp) {
            if n.rotation.x > -0.47 {
                n.rotate_x(-0.05);
            }
        }
        if keys.pressed(KeyCode::ArrowDown) {
            if n.rotation.x < 0.47 {
                n.rotate_x(0.05);
            }
        }
       
        if keys.just_pressed(KeyCode::Space) {
            n.rotation = Quat::IDENTITY
        }

    }

    let Ok((mut fv, mass)) = drone_q.get_single_mut() else {
        return;
    };

    if keys.just_pressed(KeyCode::Space) {
        println!("{}  :  {}", fv.0, mass.0);
        // fv.0 = ((9.81 * mass.0 / 4.) as i32) as _;
        fv.0 = 9.81 * mass.0;
    }

    for ev in scroll_evt.read() {
        fv.0 += ev.y * 10.;
    }

}

// use bevy_hierarchy::prelude::*;

fn set_nozzles(
    nozzles_q: Query<Entity, With<Nozzle>>,
    drone_q: Query<Entity, (With<Drone>, Without<Nozzles>)>,
    children_q: Query<&Children>,
    mut commands: Commands
) {
    if drone_q.is_empty() {
        return;
    }
   
    let nozzles: Vec<Entity> = nozzles_q.iter().collect();
    
    for drone_e in drone_q.iter() {
        let mut tmp: Vec<Entity> = Vec::new();
        for desc in children_q.iter_descendants(drone_e) {
            if nozzles.contains(&desc) {
                tmp.push(desc);
                // commands.entity(desc)
                // .insert(RigidBody::Dynamic)
                // .insert(ExternalForce::new(Vec3::Y * 20.).with_persistence(true));
            }
        }

        if tmp.len() > 0 {
            println!("set_nozzles");
            commands.entity(drone_e).insert(Nozzles(tmp));

        }
    }
}


fn set_forces(
    mut drone_q: Query<(&Transform, &mut ExternalForce,&ForceValue, &Nozzles), With<Drone>>,
    nt_q: Query<&GlobalTransform, With<Nozzle>>,
    mut gizmos: Gizmos

) {

    for (t , mut f, fv ,nozzles) in drone_q.iter_mut() {
        for nozzle_e in &nozzles.0 {
            let nt = nt_q.get(*nozzle_e).unwrap();
            let point = nt.translation();
            gizmos.ray(nt.translation(), nt.up() * 10., Color::BLUE);
            f.apply_force_at_point(nt.up() * fv.0 / 4., point, t.translation);

        }
        // f.apply_force(t.up() * fv.0);
        // f.set_force(t.up() * fv.0);
    }
}

// fn children(
//     mut q: Query<Entity, With<Nozzle>>,
//     dr: Query<Entity, With<Drone>>,
//     ch: Query<&Children>,

// ) {
//     let e = dr.single();
//     let nl:Vec<Entity> = q.iter().collect();

//     for c in ch.iter_descendants(e) {
//         if nl.contains(&c) {
//             println!("{:?}", c);
//         }
        
//     }
//     println!("--------------------");
// }

// fn parent(
//     n_q: Query<Entity, With<Nozzle>>,
//     p_q: Query<&Children>
// ) {
//     for  in  {
        
//     }
//     let e = dr.single();
//     for c in ch.iter_descendants(e) {
//         println!("{:?}", c);
//     }
// }
