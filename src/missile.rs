use std::{process::CommandArgs, time::Duration};

use bevy::{ecs::query, prelude::*, time::common_conditions::on_timer};
use bevy_rapier3d::prelude::*;
use bevy_hanabi::prelude::*;
use crate::drone;
use drone::Manual;

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Update, input);
        app.add_systems(Update, shot.run_if(on_event::<MissileShot>()));
        app.add_systems(Update, destroy.run_if(on_event::<MissileDestroy>()));
        app.add_systems(Update, clean.run_if(on_timer(Duration::from_secs(1))));
        app.add_systems(Update, collision.run_if(on_event::<CollisionEvent>()));
        app.add_event::<MissileShot>();
        app.add_event::<MissileDestroy>();
    }
}

// ---

#[derive(Component)]
pub struct Missile;

#[derive(Component)]
pub struct Blast;

#[derive(Event)]
pub struct MissileShot;

#[derive(Event)]
pub struct MissileDestroy(Entity);

use crate::LifeTime;
// ---

fn startup(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    commands.spawn((
        ParticleEffectBundle::new(effects.add(blast())),
        Blast
    ));
}

// ---

fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut ev_writer: EventWriter<MissileShot>
) {
    if keys.just_pressed(KeyCode::ControlLeft) {
        ev_writer.send(MissileShot);
    }
}

// ---

const BALL_RADIUS: f32 = 0.3;

fn shot(
    mut commands: Commands,
    drone_q: Query<&Transform, With<Manual>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    time: Res<Time>
) {
    if let Ok(drone_trans) = drone_q.get_single() {
        commands.spawn((
            PbrBundle {
                material: materials.add(Color::SILVER),
                mesh: meshes.add(Sphere::new(BALL_RADIUS)),
                transform: Transform::from_translation(drone_trans.translation + drone_trans.forward() * 5.).looking_to(drone_trans.forward().into(), Vec3::Y),
                ..default()
            },
            Missile,
            LifeTime(time.elapsed_seconds()),
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::ball(BALL_RADIUS),
            ExternalImpulse {impulse: drone_trans.forward() * 5. , torque_impulse: Vec3::ZERO},
            ActiveEvents::COLLISION_EVENTS,
        ))
        .with_children(|p| {
            p.spawn((
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(trail())),
                    transform: Transform::from_xyz(0., 0., -1.5 * BALL_RADIUS).with_rotation(Quat::from_rotation_x(f32::to_radians(90.))),
                    ..default()
                },
            ));
        })
        ;
    }

}

// ---

fn clean(
    delete_q: Query<(Entity,  &LifeTime), With<Missile>>,
    mut ev_writer: EventWriter<MissileDestroy>,    
    time: Res<Time>,
) {
    for (e,  lifetime,) in delete_q.iter() {
        if lifetime.0 + 5. < time.elapsed_seconds() {
            ev_writer.send(MissileDestroy(e));
        }
    }
}

// ---

fn destroy(
    mut commands: Commands,
    mut ev_reader: EventReader<MissileDestroy>,
    t_q: Query<&Transform, Without<Blast>>,
    mut b_q: Query<(&mut Transform, &mut EffectSpawner), With<Blast>>
) {
    for ev in ev_reader.read()   {
        commands.entity(ev.0).despawn_recursive();
        if let  Ok(ct) = t_q.get(ev.0) {
            if let Ok ((mut bt, mut bs)) = b_q.get_single_mut() {
                bt.translation = ct.translation;
                bs.reset();
            }
        }
    }
}

// ---

fn collision(
    mut collision_events: EventReader<CollisionEvent>,
    e_q: Query<Entity, With<Missile>>,
    mut ev_writer: EventWriter<MissileDestroy>
) {
    for c_ev  in  collision_events.read() {
        if let CollisionEvent::Stopped(e1, e2, _) = c_ev {
            for e0 in e_q.iter().filter(|e| {e == e1 || e == e2}) {
                ev_writer.send(MissileDestroy(e0));
                break;    
            }
        }
    }
}

// ================================ Effects =========================================

pub fn trail () ->EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(2., 2., 2., 1.));
    gradient.add_key(1., Vec4::new(10.0, 0.0, 0.0, 0.5));
    
    let color_modifier =  ColorOverLifetimeModifier {
        gradient: gradient,
    };
    let size_modifier = SetSizeModifier {
        size: Vec2::new(0.2, 0.2).into(),
        ..default()
    };

    let writer = ExprWriter::new();

    let age = writer.lit(0.).uniform(writer.lit(0.02)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.5).uniform(writer.lit(0.3)).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionCone3dModifier {
        height: writer.lit(2.).expr(),
        base_radius: writer.lit(0.2).expr(),
        top_radius: writer.lit(0.3).expr(),
        dimension: ShapeDimension::Surface,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(1.) + writer.lit(2.)).expr(),
    };

    EffectAsset::new(
        vec![32768],
        Spawner::rate(5000.0.into()),
        writer.finish(),
    )
    .with_name("trail")
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .render(color_modifier)
    .render(size_modifier)
}

// ---

pub fn blast () ->EffectAsset {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(2., 2., 2., 1.));
    gradient.add_key(0.1, Vec4::new(10.0, 10.0, 0.0, 0.1));
    
    let color_modifier =  ColorOverLifetimeModifier {
        gradient: gradient,
    };
    let size_modifier = SetSizeModifier {
        size: Vec2::new(0.1, 0.1).into(),
        ..default()
    };

    let writer = ExprWriter::new();

    let age = writer.lit(0.).uniform(writer.lit(0.02)).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);
    let lifetime = writer.lit(0.2).expr();
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    let init_pos = SetPositionSphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        radius: writer.lit(2.).expr(),
        dimension: ShapeDimension::Volume,
    };

    let init_vel = SetVelocitySphereModifier {
        center: writer.lit(Vec3::ZERO).expr(),
        speed: (writer.rand(ScalarType::Float) * writer.lit(20.)).expr(),
    };

    EffectAsset::new(
        vec![32768],
        Spawner::once(5000.0.into(), false),
        writer.finish(),
    )
    .with_name("blast")
    .init(init_pos)
    .init(init_vel)
    .init(init_age)
    .init(init_lifetime)
    .render(color_modifier)
    .render(size_modifier)
}
