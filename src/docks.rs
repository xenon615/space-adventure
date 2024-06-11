use std::time::Duration;
use crate::drone::NeedService;
use crate::drone::UnderService;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_hanabi::EffectProperties;
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle, EffectAsset};
use bevy_rapier3d::prelude::*;
use crate::effects::*;
use crate::Target;

pub struct DocksPlugin;
impl Plugin for DocksPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, scan.run_if(on_timer(Duration::from_secs(1))));
        app.add_systems(Update, (service, service_free).chain());
    }
}

// ---

#[derive(Component)]
pub struct Dock;

#[derive(Component)]
pub struct SupplyEffect;

#[derive(Component)]
pub struct Aura(Entity);

// ---

// fn spawn (
//     mut commands: Commands,
//     asset: ResMut<AssetServer>,
//     mut effects: ResMut<Assets<EffectAsset>>,
// ) {
//     let dock_handle = asset.load("models/dock.glb#Scene0");

//     for i in 0..2 {
//         let dock_position = vec3(i as f32  * 100. + 100., 15., 0.);
//         let mut aura_id = Entity::PLACEHOLDER;
    
//         let dock_id = commands.spawn((
//             SceneBundle {
//                 scene: dock_handle.clone(),
//                 transform: Transform::from_translation(dock_position),
//                 ..default()
//             },
//             Dock,
//             RigidBody::Dynamic,
//             GravityScale(0.),
//             Collider::cuboid(7.5, 2.5, 7.5),
//         ))
//         .with_children(|p| {
//             aura_id = p.spawn(
//                 ParticleEffectBundle {
//                     effect: ParticleEffect::new(effects.add(dock_aura_effect())),
//                     ..default()
//                 }
//             ).id();
//         }).id()
//         ;    
//         commands.entity(dock_id).insert(Aura(aura_id));
//         if i == 0 {
//             commands.entity(dock_id).insert(Target);
//         }
//     }

// }

fn spawn (
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {

    for i in 0..2 {
        let dock_position = vec3(i as f32  * 100. + 100., 15., 0.);
        let mut aura_id = Entity::PLACEHOLDER;
    
        let dock_id = commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(5.)),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0., 0., 0., 0.5),
                    reflectance: 1.,
                    emissive:Color::rgba(1., 4., 1., 0.5),
                    ..default()
                }),
                transform: Transform::from_translation(dock_position),
                ..default()
            },
            Dock,
            RigidBody::Dynamic,
            GravityScale(0.),
            Collider::ball(5.),
        ))
        .with_children(|p| {
            aura_id = p.spawn(
                ParticleEffectBundle {
                    effect: ParticleEffect::new(effects.add(dock_aura())),
                    ..default()
                }
            ).id();
        }).id()
        ;    
        commands.entity(dock_id).insert(Aura(aura_id));
        if i == 0 {
            commands.entity(dock_id).insert(Target);
        }
    }

}

use crate::drone::DroneEvent;

#[derive(Component)]
pub struct Client(pub Entity);

// ---

fn scan(
    clients_q: Query<(&Transform, Entity),  With<NeedService>>,
    docks_q: Query<(&Transform, Entity), (With<Dock>, Without<NeedService>)>,
    mut ev_info: EventWriter<DroneEvent>,
    mut commands: Commands,
) {
    for (client_transform, client_entity) in clients_q.iter() {
            for (dock_transform, dock_entity )  in docks_q.iter() {
            if dock_transform.translation.distance(client_transform.translation) < 50. {
                ev_info.send(DroneEvent::Service(client_entity));
                commands.entity(dock_entity).insert(Client(client_entity));
            }
        }
    }
}

// ---

// fn service(
//     docks_q: Query<(&Client, &SupplyBeam), (With<Dock>, Without<UnderService>)>,
//     mut clients_q: Query<(&Transform, Entity), (With<UnderService>, Without<SupplyEffect>)>,
//     mut ev_writer: EventWriter<DroneEvent>,
//     mut effect_q: Query< &mut EffectSpawner>,
//     mut effect_parent_q: Query<(&mut Transform, &Children), With<SupplyEffect>>,
// ) {
//     for (client, sp_e ) in docks_q.iter() {
//         if let Ok((client_transform, e)) = clients_q.get_mut(client.0) {
//             if let Ok((mut beam_transform, children)) = effect_parent_q.get_mut(sp_e.0) {
//                 beam_transform.look_at(client_transform.translation, Vec3::Y);

//                 // if let Ok(mut beam_spawn) = effect_q.get_mut(children[0]) {
//                 //     beam_spawn.reset();
//                 // }
//             }
//             // ev_writer.send(DroneEvent::SupplyFluel((e, 0.1)));
//         }
//     }
// }


// fn service(
//     docks_q: Query<(&Client, &Aura), (With<Dock>, Without<UnderService>)>,
//     clients_q: Query<&Transform>,
//     mut effect_q: Query<&mut EffectProperties>,
//     mut ev_writer: EventWriter<DroneEvent>,
// ) {
//     for (client_e, aura_e ) in docks_q.iter() {
//         if let Ok(client_transform) = clients_q.get(client_e.0) {
//             if let Ok(mut props) = effect_q.get_mut(aura_e.0) {
//                 let color = Color::NAVY.as_rgba_u32();
//                 props = EffectProperties::set_if_changed(props, "accel", 20.0.into());
//                 props = EffectProperties::set_if_changed(props, "origin", client_transform.translation.into());
//                 EffectProperties::set_if_changed(props, "p_color", color.into());
                
//             }
//         }

//         ev_writer.send(DroneEvent::SupplyFluel((client_e.0, 1.0)));
        
//     }
// }

fn service(
    docks_q: Query<(&Client, &Aura), (With<Dock>, Without<UnderService>)>,
    mut effect_q: Query<&mut EffectProperties>,
    mut ev_writer: EventWriter<DroneEvent>,
) {
    for (client_e, aura_e ) in docks_q.iter() {
        if let Ok(props) = effect_q.get_mut(aura_e.0) {
            let color = Color::rgba(10.0, 6.0, 0.0, 1.).as_rgba_u32();
            EffectProperties::set_if_changed(props, "p_color", color.into());
        }
        ev_writer.send(DroneEvent::SupplyFluel((client_e.0, 1.0)));
    }
}


fn service_free(
    mut removals: RemovedComponents<UnderService> ,
    docks_q: Query<(&Client, &Aura, Entity)>, 
    mut effect_q: Query<&mut EffectProperties>,
    mut commands: Commands
) {
    for client_en in  removals.read() {
        for (client, aura, dock_ent) in docks_q.iter() {
            if client_en == client.0 {
                if let Ok(props) = effect_q.get_mut(aura.0) {
                    let color = Color::rgba(0.0, 14.0, 4.0, 1.).as_rgba_u32();
                    EffectProperties::set_if_changed(props, "p_color", color.into());
                    commands.entity(dock_ent).remove::<Client>();
                }
            }
        }
    }
}


// fn service_free(
//     mut removals: RemovedComponents<UnderService> ,
//     docks_q: Query<(&Client, &Aura, Entity)>, 
//     mut effect_q: Query<&mut EffectProperties>,
//     mut commands: Commands
// ) {
//     for client_en in  removals.read() {

//         for (client, aura, dock_ent) in docks_q.iter() {
//             if client_en == client.0 {

//                 if let Ok(mut props) = effect_q.get_mut(aura.0) {
//                     println!("free");
//                     let color = Color::WHITE.as_rgba_u32();
//                     props = EffectProperties::set_if_changed(props, "accel", 0.0.into());
//                     props = EffectProperties::set_if_changed(props, "origin", Vec3::ZERO.into());
//                     props = EffectProperties::set_if_changed(props, "p_color", color.into());
//                     commands.entity(dock_ent).remove::<Client>();
//                 }
//             }
//         }
//     }
// }