use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_hanabi::prelude::*;
use crate::effects::{engine, steer, ship_aura};
use crate::camera::Focus;
use crate::ui::{RegisterWidgets, ULayout, UpdateWidgets, WidgetRegData, WidgetUpdateData, WType};
use crate::Target;
use crate::GameState;
use crate::docks::{Client, Dock};

// ---

pub struct DronePlugin;
impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (setup, setup_ui).run_if(in_state(GameState::Setup)));

        app.add_systems(Update, (
            input,
            movement,
            read_events,
            check_state,
            update_indicators
        ).run_if(in_state(GameState::Game)));
        app.add_event::<DroneEvent>();
        app.add_event::<DroneControl>();
    }
}

//  + Effects

#[derive(Component, PartialEq, Debug)]
pub enum PSEffect {
    Main,
    Aux,
}

#[derive(Component)]
pub struct  Effects{
    pub main: Entity,
    pub aux: Entity 
}

//  - Effects

// + UI Indicators 

const I_VELOCITY: (&str, &str) = ("vel", "Vel"); 
const I_DIST_XZ: (&str, &str) = ("d_xz","Dist XZ");
const I_DIST_Y: (&str, &str) = ("d_y","Dist Y");
const I_FLUEL: (&str, &str) = ("fluel","Fluel");
const I_DIRECTION_KEY: &str = "dir";

// - UI Indicators 

// + Markers =====================================================================================================

#[derive(Component)]
pub struct Drone;

#[derive(Component)]
pub struct NeedService;

#[derive(Component)]
pub struct Manual;

#[derive(Component)]
pub struct UnderService;

use crate::NotReady;

#[derive(Component)]
pub struct TempDroneUI;


// - Markers =====================================================================================================

// + Movement ====================================================================================================

#[derive(Component)]
pub struct Multiplier {
    linear: f32,
    angular: f32
}

// - Movement =====================================================================================================

const FLUEL_CAPACITY: f32 = 1000.;

#[derive(Component)]
pub struct Fluel(f32);
impl Fluel{
    pub fn gain(&mut self, v: f32) -> bool {
        self.0 += f32::min(v, FLUEL_CAPACITY - self.0);
        self.0 == FLUEL_CAPACITY
    }

    pub fn loss(&mut self, v: f32) -> bool {
        self.0 -= f32::min(v, self.0);
        self.0 == 0.
    }

    pub fn get(&self) -> f32 {
        self.0
    }

    pub fn percent(&self) -> f32 {
        self.0  / FLUEL_CAPACITY
    }

    fn limit(&self) -> bool {
        self.percent() < 0.2
    }

}



// + Events =======================================================================================================


#[derive(Event, PartialEq)]
pub enum DroneEvent {
    Service(Entity),
    SupplyFluel((Entity, f32))
}

#[derive(Event, PartialEq)]
pub struct DroneControl((Entity, usize, f32)); 

// - Events =======================================================================================================

const LINEAR_DAMPING_DEFAULT: f32 = 0.01;  

// ---

fn spawn(
    mut commands: Commands,
    asset: ResMut<AssetServer>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    commands.spawn((
        SceneBundle {
            scene: asset.load("models/ship2.glb#Scene0"),
            transform: Transform::from_xyz(0., 10., 0.) ,
            ..default()
        },
        Name::new("Drone"),
        Drone,
        Focus,
        Fluel(FLUEL_CAPACITY),
        RigidBody::Dynamic,
        Collider::cuboid(1.25, 0.25, 2.25),
        GravityScale(0.),
        ExternalImpulse {impulse:Vec3::ZERO, torque_impulse: Vec3::ZERO},
        Multiplier {linear: 100., angular: 10.},
        Velocity::default(),
        Damping{linear_damping: LINEAR_DAMPING_DEFAULT, angular_damping: 5.},
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
        Manual,
        NotReady
    ))
    .with_children(|parent| {
        parent.spawn((
            ParticleEffectBundle {
                effect: ParticleEffect::new(effects.add(engine())),
                transform: Transform::from_xyz(0., 0., 19.25)
                .with_rotation(Quat::from_rotation_x(f32::to_radians(90.)))
                .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            },
            PSEffect::Main,
        ));
        
        parent.spawn((
            ParticleEffectBundle {
                effect: ParticleEffect::new(effects.add(steer())),
                transform: Transform::from_xyz(1.2, 0., 1.)
                .with_rotation(Quat::from_rotation_x(f32::to_radians(90.)))
                .with_scale(Vec3::splat(0.5)),
                
                ..Default::default()
            },
            PSEffect::Aux,
        ));

        parent.spawn((
            ParticleEffectBundle {
                effect: ParticleEffect::new(effects.add(ship_aura())),
                transform: Transform::from_xyz(0., 0., 0.),
                ..Default::default()
            },
        ));
    })
    ;
    
    commands.spawn((NotReady, TempDroneUI));

} 

// ---

fn setup (
    mut commands: Commands,
    d_q: Query<(Entity, &Children), (With<NotReady>, With<Drone>)>,
    e_q: Query<(Entity, &PSEffect)>,
) {
    for (e, children) in d_q.iter() {
        let mut effects = Effects{main: Entity::PLACEHOLDER, aux: Entity::PLACEHOLDER};
        for che in children.iter() {
            if let Ok((ce, eff)) =  e_q.get(*che) {
                if *eff == PSEffect::Main {
                    effects.main = ce
                } else if *eff == PSEffect::Aux {
                    effects.aux = ce
                }
            }
        }
        commands.entity(e).insert(effects);
        commands.entity(e).remove::<NotReady>();
    }
}

// ---

fn setup_ui(
    mut writer: EventWriter<RegisterWidgets>,
    layout_q: Query<&ULayout>,
    not_ready_q: Query<Entity, (With<NotReady>, With<TempDroneUI>)>,
    mut commands: Commands,
    asset: ResMut<AssetServer>,
) {
    let Ok(nr_ent) = not_ready_q.get_single() else {
        return;
    };

    for le in layout_q.iter() {
        if *le == ULayout::Header {
            writer.send(
                RegisterWidgets(
                    vec![
                        WidgetRegData {
                            key: I_VELOCITY.0,
                            label: I_VELOCITY.1,
                            parent: ULayout::Header,
                            wtype: WType::Float,
                            image: None,
                            start: 1,
                            span: 2,
                            default: None
                        },
                        WidgetRegData {
                            key: I_DIST_XZ.0,
                            parent: ULayout::Header,
                            wtype: WType::Float,
                            label: I_DIST_XZ.1,
                            start: 3,
                            span: 2,
                            image: None,
                            default: None
                        },
                        WidgetRegData {
                            key: I_DIST_Y.0,
                            parent: ULayout::Header,
                            wtype: WType::Float,
                            label: I_DIST_Y.1,
                            start: 5,
                            span: 2,
                            image: None,
                            default: None
                        },
                        WidgetRegData {
                            key: I_FLUEL.0,
                            parent: ULayout::Header,
                            wtype: WType::Float,
                            label: I_FLUEL.1,
                            start: 7,
                            span: 2,
                            image: None,
                            default: None
                        },
                        WidgetRegData {
                            key: I_DIRECTION_KEY,
                            parent: ULayout::Header,
                            wtype: WType::Image,
                            label: "",
                            start: 9,
                            span: 1,
                            image: Some(asset.load("images/arrow.png")),
                            default: None
                        },

                    ]
                )
            );
            commands.entity(nr_ent).despawn();
        }
    }

}

// ---

fn input (
    keys: Res<ButtonInput<KeyCode>>,
    drone_q: Query<Entity , (With<Drone>, With<Manual>)>,
    mut ev_writer: EventWriter<DroneControl>
) {
    let kk = [
        KeyCode::KeyW, KeyCode::KeyS, 
        KeyCode::KeyA, KeyCode::KeyD,  
        KeyCode::ArrowDown, KeyCode::ArrowUp, 
        KeyCode::ArrowLeft, KeyCode::ArrowRight, 
        KeyCode::KeyB
    ];
    
    if keys.any_pressed(kk) {
        let Ok(e) = drone_q.get_single() else {
            return;
        };

        if keys.pressed(KeyCode::KeyW) {
            ev_writer.send(DroneControl((e, 0, 1.)));
        }

        if keys.pressed(KeyCode::KeyS) {
            ev_writer.send(DroneControl((e, 0, -1.)));
        }

        if keys.pressed(KeyCode::ArrowUp) {
            ev_writer.send(DroneControl((e, 1, 1.)));
        }

        if keys.pressed(KeyCode::ArrowDown) {
            ev_writer.send(DroneControl((e, 1, -1.)));
        }

        if keys.pressed(KeyCode::KeyD) {
            ev_writer.send(DroneControl((e, 2, 10.)));
        }

        if keys.pressed(KeyCode::KeyA) {
            ev_writer.send(DroneControl((e, 2, -10.)));
        }

        if keys.pressed(KeyCode::KeyB) {
            ev_writer.send(DroneControl((e, 3, 10.)));
        } 

        if keys.pressed(KeyCode::ArrowLeft) {
            ev_writer.send(DroneControl((e, 2, -2.)));
        } 

        if keys.pressed(KeyCode::ArrowRight) {
            ev_writer.send(DroneControl((e, 2, 2.)));
        } 

    }
    
}
    
// ---

fn movement(
    mut ev_reader: EventReader<DroneControl>,
    mut drone_q: Query< (&Transform, &mut ExternalImpulse,&mut Damping, &Multiplier, &Effects, &mut Fluel),  (With<Drone>, Without<UnderService>)>,
    mut spawners_q: Query<(&mut Transform, &mut EffectSpawner), Without<Drone>>,
    time: Res<Time>
) {
    for ev in ev_reader.read() {
        if let Ok((drone_transform, mut ei, mut dmp ,mult, effs, mut fluel)) = drone_q.get_mut(ev.0.0) {
            if fluel.get() <= 0. {
                return;
            }
            let mut fluel_loss_mult: f32 = 0.;
            if ev.0.1 == 0 {
                ei.impulse = drone_transform.forward() * mult.linear * ev.0.2 * time.delta_seconds();
                fluel_loss_mult = 0.1;
                if let Ok((mut loc_trans, mut s)) = spawners_q.get_mut(effs.main) {
                    loc_trans.translation.z = 6.1 * ev.0.2.signum();
                    loc_trans.translation.y = 0.;
                    loc_trans.rotation = Quat::from_euler(EulerRot::XYZ, ev.0.2 * PI / 2., 0., 0.);
                    s.reset();
                }
            }

            if ev.0.1 == 1 {
                ei.impulse = drone_transform.up() * mult.linear * ev.0.2   * time.delta_seconds();
                fluel_loss_mult = 0.1;
                if let Ok((mut loc_trans, mut s)) = spawners_q.get_mut(effs.main) {
                    loc_trans.translation.z = 1.;
                    loc_trans.translation.y = -1.5 * ev.0.2.signum();
                    loc_trans.rotation = if ev.0.2 > 0. {Quat::from_euler(EulerRot::XYZ, PI, 0., 0.)} else {Quat::IDENTITY};
                    s.reset();
                }
            }

            if ev.0.1 == 2 {
                ei.torque_impulse = drone_transform.up() * -ev.0.2  * time.delta_seconds() * mult.angular;
                fluel_loss_mult = 0.05;
                if let Ok((mut loc_trans, mut s)) = spawners_q.get_mut(effs.aux) {
                    loc_trans.translation.x = - ev.0.2.signum() * 5.5;
                    s.reset();
                }
            }

            if ev.0.1 == 3 {
                dmp.linear_damping = ev.0.2;
            } else {
                dmp.linear_damping = LINEAR_DAMPING_DEFAULT;
            } 

            if fluel_loss_mult > 0. {
                fluel.loss(ev.0.2.abs() * fluel_loss_mult);
            }

        }
    }
}

// ---

fn read_events (
    mut reader : EventReader<DroneEvent>,
    mut ev_writer: EventWriter<DroneControl>,
    mut commands: Commands,
    mut drone_q: Query<&mut Fluel>
) {
    for e in reader.read() {
        match e {
            DroneEvent::Service {0: e} => {
                commands.entity(*e).remove::<NeedService>();
                commands.entity(*e).insert(UnderService);

                ev_writer.send(DroneControl((*e, 3, 10.)));
            },
            DroneEvent::SupplyFluel {0: (e,v) } => {
                if let Ok(mut fluel ) = drone_q.get_mut(*e) {
                    if fluel.gain(*v) {
                        commands.entity(*e).remove::<UnderService>();
                    }
                }
            }
        }
    }
}

// ---

fn check_state (
    mut commands: Commands,
    drone_q: Query<(Entity, &Fluel, &Transform) , With<Drone>> ,
    docks_q:Query<(&Transform, Entity), (With<Dock>, Without<Client>)>,
    target_q: Query<Entity, With<Target>>  
) {
    if let Ok((drone_e, fluel, drone_trans))  = drone_q.get_single() {
        if fluel.limit() {
            commands.entity(drone_e).insert(NeedService);
            let mut candidate = Entity::PLACEHOLDER;
            let mut min_distance =  f32::MAX;
            for (dock_trans, dock_e) in docks_q.iter() {
                let distance = drone_trans.translation.distance_squared(dock_trans.translation);
                if  distance < min_distance {
                    min_distance = distance;
                    candidate = dock_e;
                }
            } 

            if candidate != Entity::PLACEHOLDER {
                if let Ok(old_target) = target_q.get_single() {
                    commands.entity(old_target).remove::<Target>();
                    commands.entity(candidate).insert(Target);
                }
            }
            
        }
    }
}

// ---

fn update_indicators(
    drone_q: Query<(&Velocity, &Transform, &Fluel), (With<Drone>, With<Focus>,  Without<Target>)>,
    target_q: Query<&Transform, (With<Target>, Without<Drone>, Without<Focus>)>,
    mut writer: EventWriter<UpdateWidgets>
) {
    let target_translation = if let Ok(target_transform) =  target_q.get_single()  {
        target_transform.translation
    } else {
        Vec3::ZERO
    };

    let Ok((v, drone_transform, fluel)) = drone_q.get_single() else {
        return;
    };
    let to_target = target_translation - drone_transform.translation;

    let to_target_xz = to_target.normalize().reject_from_normalized(Vec3::Y);
    let forward_xz: Vec3 = drone_transform.forward().into();
    let dot = to_target_xz.dot(forward_xz);
    let sign = to_target_xz.cross(forward_xz).y.signum();
    let angle = dot.acos() * sign;

    writer.send(UpdateWidgets(
        vec![
            WidgetUpdateData::from_key_value(I_VELOCITY.0, v.linvel.length()),
            WidgetUpdateData::from_key_value(I_DIST_XZ.0, to_target.reject_from(Vec3::Y).length()),
            WidgetUpdateData::from_key_value(I_DIST_Y.0, drone_transform.translation.y -  target_translation.y),
            WidgetUpdateData::from_key_value_color(I_FLUEL.0, fluel.get(), if fluel.limit() {Color::ORANGE_RED} else {Color::YELLOW_GREEN}),
            WidgetUpdateData::from_key_value(I_DIRECTION_KEY, angle),
        ]
    ));

   
    
}
