use std::time::Duration;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_rapier3d::prelude::*;

use crate::missile::Missile;
use crate::Health;
pub struct AsteroidsPlugin;
impl Plugin for AsteroidsPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (collision, check).chain().run_if(on_event::<CollisionEvent>()));
        app.add_systems(Update, cleanup.run_if(on_timer(Duration::from_secs(5))));
    }
}

// ==================================

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct AsteroidType(usize);


const ASTEROID_HEALTH: f32 = 10.;

#[derive(Resource)] 
pub struct MatMeshes(Vec<(Handle<Mesh>, Handle<StandardMaterial>)>);

use crate::LifeTime;

// ==================================

fn spawn(
    mut commands: Commands,
    assets: ResMut<AssetServer>,
) {
    let mut mm:Vec<(Handle<Mesh>, Handle<StandardMaterial>)> = Vec::new();
    let range = 400..1000;
    let dev_range = -45..45;
     
    for i in 0..3 {
        let mesh = assets.load(format!("models/asteroids.glb#Mesh{}/Primitive0", i));
        let material = assets.load(format!("models/asteroids.glb#Material{}", i));
        mm.push((
            mesh.clone(), material.clone() 
        ));


        for _j in 0..20 {
            let initial_pos = Vec3::new(fastrand::i32(range.clone()) as _ , fastrand::i32(range.clone()) as _, fastrand::i32(range.clone()) as _);
            let target = Vec3::splat(0.) + Vec3::new(fastrand::i32(dev_range.clone()) as _ , fastrand::i32(dev_range.clone()) as _, fastrand::i32(dev_range.clone()) as _);
            commands.spawn((
                PbrBundle {
                    mesh : mesh.clone(),
                    material : material.clone(),
                    transform: Transform::from_translation(initial_pos),
                    ..default()
                },
                Asteroid,
                AsteroidType(i),
                Health(ASTEROID_HEALTH),
                RigidBody::Dynamic,
                GravityScale(0.),
                Collider::ball(10.),
                ActiveEvents::COLLISION_EVENTS,
                ExternalImpulse{impulse: (target - initial_pos) * 50., torque_impulse: Vec3::Y * 2.}
            ));    
        }
    }
    commands.insert_resource(MatMeshes(mm));
} 

// ---

fn collision(
    mut collision_events: EventReader<CollisionEvent>,
    mut e_q: Query<(Entity, &mut Health), With<Asteroid>>,
    m_q: Query<Entity, With<Missile>>
) {
    for c_ev  in  collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = c_ev {
            for (_, mut h) in e_q.iter_mut().filter(|(e, _)| {e == e1 || e == e2}) {
                if m_q.contains(*e1) || m_q.contains(*e2) {
                    h.0 -= 10.;
                } 
                // h.0 -= 10.;
                break;    
            }
        }
    }
}

// ---

fn check(
    mut commands: Commands,
    e_q: Query<(Entity, &Health, &AsteroidType, &Transform), With<Asteroid>>,
    mm: Res<MatMeshes>,
    time: Res<Time>
) {
    
    for (e, h, at, tr) in e_q.iter() {
        if h.0 <= 0. {
            commands.entity(e).despawn_recursive();

            let fragment_count =  fastrand::u32(10..20) as usize;  
            let lattice = fibonacci_sphere(fragment_count);
            for i in 0..= fragment_count {
                let scale = fastrand::u32(1..6) as f32 / 10.;
                commands.spawn((
                    PbrBundle {
                        mesh : mm.0[at.0].0.clone(),
                        material: mm.0[at.0].1.clone(),
                        transform: Transform::from_translation(tr.translation + lattice[i]).with_scale(Vec3::splat(scale) * tr.scale.x),
                        ..default()
                    },
                    Collider::ball(10. * scale * tr.scale.x),
                    Damping{linear_damping:0.5, angular_damping: 0.3},
                    Asteroid,
                    AsteroidType(at.0),
                    GravityScale(0.),
                    RigidBody::Dynamic,
                    Health(10.),
                    LifeTime(time.elapsed_seconds()),
                    ExternalImpulse{impulse: lattice[i] * 5., torque_impulse: lattice[i]}
                ));
            }
        }
    }
}

// ---

fn cleanup(
    mut commands: Commands,
    delete_q: Query<(Entity,  &LifeTime), With<Asteroid>>,
    time: Res<Time>,
) {
    for (e,  lifetime,) in delete_q.iter() {
        if lifetime.0 + 15. < time.elapsed_seconds() {
            commands.entity(e).despawn_recursive();
        }
    }
}

// ---

fn fibonacci_sphere(count: usize) -> Vec<Vec3> {
    let phi = std::f32::consts::PI * (5.0_f32.sqrt() - 1.);
    (0..= count).map(|i| {
        let y = 1. - (i as f32 / (count - 1) as f32) * 2.;  
        let radius = (1. - y * y).sqrt();
        let theta = phi * i as f32;
        let x = theta.cos() * radius;
        let z = theta.sin() * radius;
        Vec3::new(x, y, z)
    }).collect() 
} 
