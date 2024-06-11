use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::camera::CamViewState;
use crate::{Target, drone::{Drone, Supplies},camera::{Focus, Cam}};
pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update, (
            update_indicators, 
            read_messages,
            move_crosshair.run_if(not(in_state(CamViewState::Free)))
        ));

        app.add_systems(OnEnter(CamViewState::Free), show_crosshair);
        app.add_systems(OnExit(CamViewState::Free), show_crosshair);

    }
}

#[derive(Component, PartialEq, Debug)]
pub enum  UIIndicator {
    Velocity,
    Distance,
    Direction,
    Elevation,
    Fluel,
    Message,
}

#[derive(Component)]
pub struct Crosshair;

fn spawn(
    mut commands: Commands,
    asset: ResMut<AssetServer>
) {

    commands.spawn(
        NodeBundle {
            style: Style {
                width: Val::Vw(100.),
                height:Val::Vh(100.),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Stretch,
                ..default()
            },
            ..default() 
        },
    )
    .with_children(|p| {
        p.spawn(
            NodeBundle {
                style: Style {
                    height: Val::Px(100.),
                    width: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            }
        )

        .with_children(|p| {
                p.spawn((
                    TextBundle::from_section(
                        "", 
                        TextStyle {
                            color: Color::WHITE,
                            font_size: 30.,
                            ..default()
                        }
                    ),
                    UIIndicator::Velocity
                ));
            
                p.spawn((
                    TextBundle::from_section(
                        "", 
                        TextStyle {
                            color: Color::GREEN,
                            font_size: 30.,
                            ..default()
                        }
                    ),
                    UIIndicator::Distance
                ));

                p.spawn((
                    TextBundle::from_section(
                        "", 
                        TextStyle {
                            color: Color::WHITE,
                            font_size: 30.,
                            ..default()
                        }
                    ),
                    UIIndicator::Elevation
                ));

                p.spawn((
                    TextBundle::from_section(
                        "", 
                        TextStyle {
                            color: Color::RED,
                            font_size: 30.,
                            ..default()
                        }
                    ),
                    UIIndicator::Fluel
                ));

                p.spawn(
                    NodeBundle {
                        style : Style {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            ..default()
                        },
                        ..default()
                    }
                )
                .with_children(|n| {
                    n.spawn((
                        ImageBundle {
                            image: UiImage::new(asset.load("images/arrow.png")),
                            ..default()
                        },
                        UIIndicator::Direction
                    ));
                }) 
                ;
            })
            ;
        p.spawn(
            NodeBundle {
                style: Style {
                    flex_grow: 1.,
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            }
        );

        p.spawn(
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::SpaceEvenly,
                    ..default()
                },
                ..default()
            }
        )
        .with_children(|p| {
            p.spawn((
                TextBundle::from_section(
                    "", 
                    TextStyle {
                        color: Color::GOLD,
                        font_size: 30.,
                        ..default()
                    }
                ),
                UIIndicator::Message
            ));
        })
        ;

        p.spawn((
            ImageBundle {
                style: Style {
                    width: Val::Px(100.),
                    height: Val::Px(100.),
                    position_type: PositionType::Absolute,
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    ..default()
                },
                image: UiImage::new(asset.load("images/crosshair.png")),
                ..default()
            },
            Crosshair
        ));
    })    
    
    ;

} 

fn update_indicators(
    drone_q: Query<(&Velocity, &Transform, &Supplies), (With<Drone>, With<Focus>,  Without<Target>)>,
    mut i_q: Query<(&mut Text, &UIIndicator)>,
    mut a_q: Query<(&mut Transform, &UIIndicator), (Without<Drone>, Without<Target>, Without<Focus>)>,
    target_q: Query<&Transform, (With<Target>, Without<Drone>, Without<Focus>)>,
) {
    let Ok(target_transform) =  target_q.get_single() else {
        return;
    };

    let Ok((v, drone_transform, supplies)) = drone_q.get_single() else {
        return;
    };


    let to_target = target_transform.translation - drone_transform.translation;

    for  (mut text, itype) in i_q.iter_mut() {
        match itype {
            UIIndicator::Velocity => {
                text.sections[0].value = format!("Vel: {:.2}",v.linvel.length())
            },
            UIIndicator::Distance => {
                text.sections[0].value = format!("Dist XZ: {:.2}",to_target.reject_from(Vec3::Y).length())
            },

            UIIndicator::Elevation => {
                text.sections[0].value = format!("Dist Y: {:.2}", drone_transform.translation.y -  target_transform.translation.y)
            },

            UIIndicator::Fluel=> {
                text.sections[0].value = format!("Fluel: {:.2}", supplies.fluel_get());
                text.sections[0].style.color = if supplies.fluel_limit() {Color::ORANGE_RED} else {Color::YELLOW_GREEN};
            },
            _ => ()
        }
    }

    for  (mut trans, itype) in a_q.iter_mut() { 
        if *itype == UIIndicator::Direction {
            let to_target_xz = to_target.normalize().reject_from_normalized(Vec3::Y);
            let forward_xz: Vec3 = drone_transform.forward().into();
            let dot = to_target_xz.dot(forward_xz);
            let sign = to_target_xz.cross(forward_xz).y.signum();
            let angle = dot.acos() * sign;
            trans.rotation = Quat::from_rotation_z(angle) ;    
        }
        
    }
}

use crate::GameMessage;

// ---

fn read_messages(
    mut i_q: Query<(&mut Text, &UIIndicator)>,
    mut ev_reader: EventReader<GameMessage>
) {
    for e in ev_reader.read() {
        for (mut text, _itype) in i_q.iter_mut().filter(|(_, itype0)| {
            **itype0 == UIIndicator::Message
        }) {
            text.sections[0].value = e.0.clone();
        }
    }
}

// ---

fn move_crosshair(
    focus_q: Query<&Transform, With<Focus>>,   
    mut crosshair_q: Query<&mut Style, With<Crosshair>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Cam>>,
) {
    for (camera, camera_transform) in camera_query.iter() {
        for focus_trans  in focus_q.iter() {
            for mut style in crosshair_q.iter_mut() {
                let far = focus_trans.translation + focus_trans.forward() * 100.;
                match camera.world_to_viewport(camera_transform, far)
                {
                    Some(coords) => {
                        style.left = Val::Px(coords.x - 50.);
                        style.top = Val::Px(coords.y - 50.);
                    },
                    None => ()
                }
            }
        }
    }
}

// ---

fn show_crosshair(
    mut crosshair_q: Query<&mut Visibility, With<Crosshair>>,
) {
    if let Ok(mut v) = crosshair_q.get_single_mut() {
        *v = match *v {
            Visibility::Hidden => Visibility::Inherited,
            Visibility::Inherited => Visibility::Hidden,
            Visibility::Visible => Visibility::Visible
        }
    }
}
