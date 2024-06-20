use std::f32::consts::PI;
use std::time::Duration;
use crate::drone::NeedService;
use crate::drone::UnderService;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy_hanabi::EffectProperties;
use bevy_hanabi::{ParticleEffect, ParticleEffectBundle, EffectAsset};
use bevy_rapier3d::prelude::*;
use crate::effects::*;

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

const DOCKS_COUNT: usize = 16;
const DOCKS_RADIUS: f32 = 600.;

fn spawn (
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let angle_step = 2. * PI / DOCKS_COUNT as f32; 
    let mut angle: f32 = 0.;
    for _i in 0 .. DOCKS_COUNT {
        let dock_position = Vec3::new(angle.cos() * DOCKS_RADIUS, 100., angle.sin() * DOCKS_RADIUS);
        angle += angle_step;
        

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
            RigidBody::Fixed,
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

// ---

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