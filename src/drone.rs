use std::f32::consts::PI;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_hanabi::prelude::*;
use crate::effects::{engine, steer,ship_aura};
use crate::camera::Focus;
use crate::Target;
use crate::GameState;
use crate::docks::{Client, Dock};

// ---

pub struct DronePlugin;
impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, setup.run_if(in_state(GameState::Setup)));

        app.add_systems(Update, (
            input,
            movement,
            read_events,
            check_state,
        ).run_if(not(in_state(GameState::Setup))));
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

// + Markers =====================================================================================================

#[derive(Component)]
pub struct Drone;

#[derive(Component)]
pub struct NeedService;

#[derive(Component)]
pub struct Manual;

#[derive(Component)]
pub struct UnderService;

#[derive(Component)]
pub struct Braking;


// - Markers =====================================================================================================

// + Movement ====================================================================================================

#[derive(Component)]
pub struct Multiplier {
    linear: f32,
    angular: f32
}

// - Movement =====================================================================================================

#[derive(Component)]
pub struct Supplies {
    fluel: (f32, f32)
}

impl Supplies {
    pub fn new(fluel: f32) -> Self{
        Self{fluel:(fluel, 0.)}
    }

    pub fn fluel_gain(&mut self, v: f32) -> bool {
        self.fluel.1 -= f32::min(v, self.fluel.1);
        self.fluel.1 == 0.
    }

    pub fn fluel_loss(&mut self, v: f32) -> bool {
        self.fluel.1 += f32::min(v, self.fluel.0 - self.fluel.1);
        self.fluel.0 ==  self.fluel.1
    }

    pub fn fluel_get(&self) -> f32 {
        self.fluel.0 - self.fluel.1
    }

    pub fn fluel_percent(&self) -> f32 {
        (self.fluel.0 - self.fluel.1) / self.fluel.0
    }

    pub fn fluel_limit(&self) -> bool {
        self.fluel_percent() < 0.2

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
            transform: Transform::from_xyz(50., 15., 50.).looking_at(Vec3::new(100., 15., 0.), Vec3::Y),
            ..default()
        },
        Name::new("Drone"),
        Drone,
        Focus,
        Supplies::new(1000.),
        RigidBody::Dynamic,
        Collider::cuboid(1.25, 0.25, 2.25),
        GravityScale(0.),
        ExternalImpulse {impulse:Vec3::ZERO, torque_impulse: Vec3::ZERO},
        Multiplier {linear: 100., angular: 10.},
        Velocity::default(),
        Damping{linear_damping: LINEAR_DAMPING_DEFAULT, angular_damping: 5.},
        LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
    ))
    .insert((
        Manual,
        NeedService
    ))
    .with_children(|parent| {
        parent.spawn((
            ParticleEffectBundle {
                effect: ParticleEffect::new(effects.add(engine())),
                transform: Transform::from_xyz(0., 0., 2.25)
                .with_rotation(Quat::from_rotation_x(f32::to_radians(90.)))
                .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                ..Default::default()
            },
            PSEffect::Main,
        ));
        
        parent.spawn((
            ParticleEffectBundle {
                effect: ParticleEffect::new(effects.add(steer())),
                transform: Transform::from_xyz(1.2, 0., 0.)
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
    

} 

// ---

fn setup (
    mut commands: Commands,
    d_q: Query<(Entity, &Children), (Without<Effects>, With<Drone>)>,
    e_q: Query<(Entity, &PSEffect)>,
    mut next: ResMut<NextState<GameState>>
) {
    
    for (e, children) in d_q.iter() {
        let mut temp = Effects{main: Entity::PLACEHOLDER, aux: Entity::PLACEHOLDER};
        for che in children.iter() {
            if let Ok((ce, eff)) =  e_q.get(*che) {
                if *eff == PSEffect::Main {
                    temp.main = ce
                } else if *eff == PSEffect::Aux {
                    temp.aux = ce
                }
            }
        }
        commands.entity(e).insert(temp);
        next.set(GameState::Game);
    }
}

// ---

fn input (
    keys: Res<ButtonInput<KeyCode>>,
    mut drone_q: Query<Entity , (With<Drone>, With<Manual>)>,
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
        let Ok(e) = drone_q.get_single_mut() else {
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
            ev_writer.send(DroneControl((e, 2, -5.)));
        } 

        if keys.pressed(KeyCode::ArrowRight) {
            ev_writer.send(DroneControl((e, 2, 5.)));
        } 

    }
    
}
    
// ---

fn movement(
    mut ev_reader: EventReader<DroneControl>,
    mut drone_q: Query< (&Transform, &mut ExternalImpulse,&mut Damping, &Multiplier, &Effects, &mut Supplies),  (With<Drone>, Without<UnderService>)>,
    mut spawners_q: Query<(&mut Transform, &mut EffectSpawner), Without<Drone>>,
    time: Res<Time>
) {
    for ev in ev_reader.read() {
        if let Ok((drone_transform, mut ei, mut dmp ,mult, effs, mut supplies)) = drone_q.get_mut(ev.0.0) {
            if supplies.fluel_get() <= 0. {
                return;
            }
            let mut fluel_loss: f32 = 0.;
            if ev.0.1 == 0 {
                ei.impulse = drone_transform.forward() * mult.linear * ev.0.2 * time.delta_seconds();
                fluel_loss = 0.1;
                if let Ok((mut loc_trans, mut s)) = spawners_q.get_mut(effs.main) {
                    loc_trans.translation.z = 2.25 * ev.0.2.signum();
                    loc_trans.rotation = Quat::from_euler(EulerRot::XYZ, ev.0.2 * PI / 2., 0., 0.);
                    s.reset();
                }
            }

            if ev.0.1 == 1 {
                ei.impulse = drone_transform.up() * mult.linear * ev.0.2   * time.delta_seconds();
                fluel_loss = 0.1;
                if let Ok((mut loc_trans, mut s)) = spawners_q.get_mut(effs.main) {
                    loc_trans.translation.z = 0.;
                    loc_trans.rotation = if ev.0.2 > 0. {Quat::from_euler(EulerRot::XYZ, PI, 0., 0.)} else {Quat::IDENTITY};
                    s.reset();
                }
            }

            if ev.0.1 == 2 {
                ei.torque_impulse = drone_transform.up() * -ev.0.2  * time.delta_seconds() * mult.angular;
                fluel_loss = 0.05;
                if let Ok((mut loc_trans, mut s)) = spawners_q.get_mut(effs.aux) {
                    loc_trans.translation.x = - ev.0.2.signum() * 1.5;
                    s.reset();
                }
            }

            if fluel_loss > 0. {
                supplies.fluel_loss(fluel_loss);
            }

            if ev.0.1 == 3 {
                dmp.linear_damping = ev.0.2;
            } else {
                dmp.linear_damping = LINEAR_DAMPING_DEFAULT;
            } 
        }
    }
}

// ---

fn read_events (
    mut reader : EventReader<DroneEvent>,
    mut ev_writer: EventWriter<DroneControl>,
    mut commands: Commands,
    mut drone_q: Query<&mut Supplies>
) {
    for e in reader.read() {
        match e {
            DroneEvent::Service {0: e} => {
                commands.entity(*e).remove::<NeedService>();
                commands.entity(*e).insert(UnderService);

                ev_writer.send(DroneControl((*e, 3, 10.)));
            },
            DroneEvent::SupplyFluel {0: (e,v) } => {
                if let Ok(mut sup ) = drone_q.get_mut(*e) {
                    if sup.fluel_gain(*v) {
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
    drone_q: Query<(Entity, &Supplies, &Transform) , With<Drone>> ,
    docks_q:Query<(&Transform, Entity), (With<Dock>, Without<Client>)>,
    target_q: Query<Entity, With<Target>>  
) {
    if let Ok((drone_e, sup, drone_trans))  = drone_q.get_single() {
        if sup.fluel_limit() {
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

