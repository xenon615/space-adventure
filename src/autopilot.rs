// use avian3d::parry::na::ComplexField;
use avian3d::prelude::LinearVelocity;
// use bevy::gizmos::gizmos;
use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;


use crate::drone::{Drone, Manual, DroneControl};
use crate::Target;

#[derive(Component)]
pub struct AutoPilot;

pub struct AutoPilotPlugin;
impl Plugin for AutoPilotPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, input.run_if(on_event::<KeyboardInput>()))
        .add_systems(Update, do_auto)

        ;
    }
}

// ---

fn input(
    keys: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    drome_q: Query<(Entity, Option<&AutoPilot>), (With<Drone>, With<Manual>)>

) {
    if keys.just_pressed(KeyCode::KeyV) {
        if let Ok((de, oap)) = drome_q.get_single() {
            if let Some(_) = oap {
                commands.entity(de).remove::<AutoPilot>();
            } else {
                commands.entity(de).insert(AutoPilot);
            }
        }
    }
}

// ---

fn do_auto(
    drone_q: Query<(Entity, &Transform, &LinearVelocity), With<AutoPilot>>,
    target_q: Query<&Transform, (With<Target>, Without<AutoPilot>)>,
    mut ev_writer: EventWriter<DroneControl>,
    // mut gizmos: Gizmos,
    time: Res<Time>
    
) {
    if let Ok(target_transform) = target_q.get_single() {
        if let Ok((de, dt, lv))  =  drone_q.get_single() {
            let to_target = target_transform.translation - dt.translation;
            let to_target_xy = to_target.reject_from(Vec3::Y);

            let dot = to_target_xy.normalize().dot(dt.right().into());
            if dot.abs() > 0.01 {
                ev_writer.send(DroneControl(de, 2, dot * time.delta_seconds() * 100.));    
            }

            if (lv.y.abs() < 5. || lv.y.signum() != to_target.y.signum()) && to_target.y.abs() > 5.   {
                ev_writer.send(DroneControl(de, 1, to_target.y * time.delta_seconds()));
            }

            // gizmos.ray(dt.translation, to_target, Color::srgb(1., 1., 0.));
            // gizmos.ray(dt.translation, lv.0, Color::srgb(1., 0., 0.));

            if lv.length_squared() > 1000. {
                ev_writer.send(DroneControl(de, 3, 1.));
            }
            
            if  to_target_xy.length_squared() > 2000. && lv.0.reject_from(Vec3::Y).length_squared() < 500.   {
                ev_writer.send(DroneControl(de, 0, 10. * time.delta_seconds()));
            }

            
        }
    }
}