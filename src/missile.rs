use bevy::prelude::*;
use avian3d::prelude::*;
use bevy_hanabi::prelude::*;
use crate::{drone, GameState, NotReady};
use drone::Manual;

pub struct MissilePlugin;
impl Plugin for MissilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
        app.add_systems(Update, setup.run_if(in_state(GameState::Setup)));
        app.add_systems(Update, (input, ));
        app.add_systems(Update, shot.run_if(on_event::<MissileShot>()));
        app.add_systems(Update, destroy.run_if(on_event::<MissileDestroy>()));
        app.add_systems(Update, collision.run_if(on_event::<CollisionEnded>()));

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
use crate::effects::{blast, trail};
use crate::drone::Drone;
use crate::ui::{RegisterWidgets, ULayout, UpdateWidgets, WType, WidgetRegData, WidgetUpdateData};


#[derive(Component)]
pub struct MissileTempMarker;

const MISSILES_CAPACITY: i32 = 10;
const I_MISSILES: (&str, &str) = ("m", "M");
#[derive(Component)]
pub struct Missiles(i32);

// ---

fn startup(
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    commands.spawn((
        ParticleEffectBundle::new(effects.add(blast())),
        Blast
    ));
    commands.spawn((NotReady, MissileTempMarker));
}

// ---

fn setup (
    mut commands: Commands,
    drones_q: Query<Entity, (With<Drone>, Without<Missiles>)>,
    check_q: Query<Entity, (With<NotReady>, With<MissileTempMarker>)>,
    mut writer: EventWriter<RegisterWidgets>,
) {
    if drones_q.is_empty() {
        if let Ok(e) = check_q.get_single() {
            commands.entity(e).despawn();
        }
        return;
    }    
    for de in drones_q.iter() {
        commands.entity(de).insert(Missiles(MISSILES_CAPACITY));
    }
    writer.send(
        RegisterWidgets(
            vec![
                WidgetRegData {
                    key: I_MISSILES.0,
                    label: I_MISSILES.1,
                    parent: ULayout::SidebarLeft,
                    wtype: WType::Integer,
                    image: None,
                    start: 1,
                    span: 2,
                    default: Some(MISSILES_CAPACITY as f32)
                },
            ]
        )
    );

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
    mut drone_q: Query<(&Transform, &mut Missiles), With<Manual>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut effects: ResMut<Assets<EffectAsset>>,
    time: Res<Time>,
    mut writer: EventWriter<UpdateWidgets>
) {
    if let Ok((drone_trans, mut missiles)) = drone_q.get_single_mut() {
        if missiles.0 == 0  {
            return;
        }

        missiles.0 -= 1; 
        writer.send(UpdateWidgets(vec![
            WidgetUpdateData::from_key_value(I_MISSILES.0, missiles.0 as f32)
        ])); 
        commands.spawn((
            PbrBundle {
                material: materials.add(Color::srgba(0.3, 0.3, 0.3, 0.5)),
                mesh: meshes.add(Sphere::new(BALL_RADIUS)),
                transform: Transform::from_translation(drone_trans.translation + drone_trans.forward() * 15.).looking_to(drone_trans.forward(), Vec3::Y),
                ..default()
            },
            Missile,
            LifeTime(time.elapsed_seconds() + 10.),
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::sphere(BALL_RADIUS),
            ExternalImpulse::new(drone_trans.forward() * 5.),
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
    mut collision_events: EventReader<CollisionEnded>,
    e_q: Query<Entity, With<Missile>>,
    mut ev_writer: EventWriter<MissileDestroy>
) {
    for CollisionEnded(e1, e2)  in  collision_events.read() {
        for e0 in e_q.iter().filter(|e| {e == e1 || e == e2}) {
            ev_writer.send(MissileDestroy(e0));
            break;    
        }
    }    
}