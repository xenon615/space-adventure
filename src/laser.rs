use bevy::prelude::*;
// use bevy_rapier3d::{prelude::*, rapier::geometry::CollisionEventFlags};
use avian3d::prelude::*;
use bevy_hanabi::prelude::*;

pub struct LaserPlugin;
impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, setup.run_if(in_state(GameState::Setup)));
        app.add_systems(Update, input);
        app.add_systems(Update, shot.run_if(on_event::<LaserShot>()));
        app.add_event::<LaserShot>();
    }
}

use crate::{drone::{Manual, Fluel}, effects::{laser, small_blast}, GameState, Health};

// ---

#[derive(Event)]
pub struct LaserShot(Entity);

use crate::NotReady;
use crate::drone::Drone;

#[derive(Component)]
pub struct LaserTempMarker;

#[derive(Component)]

pub struct LaserEffects([Entity; 4]);

const LASER_DPS: f32 = 0.05; 
const LASER_SHOT_COST: f32 = 0.1;

// ---

fn spawn(
    mut commands: Commands,    
) {
    commands.spawn((NotReady, LaserTempMarker));
}

// ---

fn setup (
        mut commands: Commands,
        check_q: Query<Entity, (With<NotReady>, With<LaserTempMarker>)>,
        drones_q: Query<Entity, (With<Drone>, Without<LaserEffects>)>,
        mut effects: ResMut<Assets<EffectAsset>>,
    ) {
        if drones_q.is_empty() {
            if let Ok(e) = check_q.get_single() {
                commands.entity(e).despawn();
            }
            return;
        }
    
        let muzzle_handle = effects.add(laser());
        let blast_handle = effects.add(small_blast());
    
        for drone_entity in drones_q.iter() {
            let mut effect_ents = [Entity::PLACEHOLDER;4];

            for i in 0..2 {
                let sign = if i == 0  {-1.} else {1.};
                let muzzle = commands.spawn(
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(muzzle_handle.clone()),
                        transform: Transform::from_xyz(sign * 5.2, 0.,0.,).with_rotation(Quat::from_rotation_x(f32::to_radians(-90.))),
                        ..default()
                    },
                ).id();
                
                commands.entity(drone_entity).add_child(muzzle);
                effect_ents[i] = muzzle;
            }
    
            for i in 2..4 {
                let blast = commands.spawn((
                    ParticleEffectBundle::new(blast_handle.clone()),
                )).id();

                effect_ents[i] = blast;
            }
            commands.entity(drone_entity).insert(LaserEffects(effect_ents));
        }
    
    }

// ---

fn input (
    keys: Res<ButtonInput<KeyCode>>,
    drone_q: Query<Entity , With<Manual>>,
    mut ev_writer: EventWriter<LaserShot>
) {
    if keys.pressed(KeyCode::ControlRight) {
        if let Ok(de) = drone_q.get_single() {
            ev_writer.send(LaserShot(de));
        }
    }
}

// ---

fn shot(
    mut ev_reader: EventReader<LaserShot>,
    // mut ev_writer: EventWriter<CollisionEvent>,
    mut ev_writer: EventWriter<CollisionEnded>,
    // rapier_context: Res<RapierContext>,
    spatial: SpatialQuery,
    mut drone_q: Query<(&Transform, &LaserEffects, &mut Fluel)>,
    mut effects_q: Query<(&mut Transform, &mut EffectSpawner), Without<LaserEffects>>,
    mut victim_q: Query<Option<&mut Health>> ,
) {
    for ev in ev_reader.read() {
        let Ok((drone_transform, effect_entities, mut fluel)) = drone_q.get_mut(ev.0) else {
            continue;
        };
        if let Ok(mut effects) = effects_q.get_many_mut(effect_entities.0) {
    
            let mut i =  0;
            for ef in &mut effects {
                if i < 2 {
                    ef.1.reset();
                    fluel.loss(LASER_SHOT_COST);
                } else {
                    let shift = (if i == 2 {-1.} else {1.}) * 5.2;
                    let ray_origin = drone_transform.translation + drone_transform.right() * shift  + drone_transform.forward() * 5.; 

                    // if let Some((e, toi)) = rapier_context.cast_ray(
                    //     ray_origin, 
                    //     drone_transform.forward().into(),
                    //     200.,
                    //     true, 
                    //     QueryFilter::default()
                    // ) {
                    //     ef.0.translation = ray_origin + drone_transform.forward() * toi;
                    //     ef.1.reset();
                    //     if let Ok(oh) = victim_q.get_mut(e) {
                    //         if let Some(mut h) = oh {
                    //             ev_writer.send(CollisionEvent::Started(Entity::PLACEHOLDER, e, CollisionEventFlags::all()));
                    //             h.0 -= LASER_DPS;
                    //         }
                    //     }
                    // }

                    if let Some(hit) = spatial.cast_ray(
                        ray_origin, 
                        drone_transform.forward().into(),
                        200.,
                        true, 
                        SpatialQueryFilter::default()
                    ) {
                        
                        ef.0.translation = ray_origin + drone_transform.forward() * hit.time_of_impact;
                        ef.1.reset();
                        if let Ok(oh) = victim_q.get_mut(hit.entity) {
                            if let Some(mut h) = oh {
                                ev_writer.send(CollisionEnded(Entity::PLACEHOLDER, hit.entity));
                                h.0 -= LASER_DPS;
                            }
                        }
                    }




                }
                i += 1; 
            }
        }
    }
}
