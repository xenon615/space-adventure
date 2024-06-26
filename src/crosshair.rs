use bevy::prelude::*;

use crate::{ui::ULayout, GameState, NotReady};
use crate::camera::{CamViewState, Focus};
use crate::Target;
use crate::camera::Cam;

pub struct CrosshairPlugin;
impl Plugin for CrosshairPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
        app.add_systems(Update,setup.run_if(in_state(GameState::Setup)));
        app.add_systems(OnEnter(CamViewState::Free), show_it);
        app.add_systems(OnExit(CamViewState::Free), show_it);
        app.add_systems(Update, move_it.run_if(not(in_state(CamViewState::Free))));
    }
}

// ---

#[derive(Component)]
pub struct CrosshairTempMarker;

#[derive(Component)]
pub struct Crosshair;

// ---

fn spawn (
    mut commands: Commands,
) {
    commands.spawn((CrosshairTempMarker, NotReady));
}

// ---

fn setup (
    mut commands: Commands,
    check_q: Query<Entity, (With<NotReady>, With<CrosshairTempMarker>)>,
    parent_q: Query<(Entity, &ULayout)>,
    asset: ResMut<AssetServer>,
) {
    if check_q.is_empty() {
        return;
    }

    for (pe, pl)  in  parent_q.iter() {
        if *pl == ULayout::Wrapper {
            let crh = commands.spawn((
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
            )).id()
            ;
            commands.entity(pe).add_child(crh);
            let e = check_q.get_single().unwrap();
            commands.entity(e).despawn();
        }
    }   

}

// ---

fn move_it(
    focus_q: Query<&Transform, (With<Focus>, Without<Target>)>,   
    target_q: Query<&Transform, (With<Target>, Without<Focus>)>,
    mut crosshair_q: Query<&mut Style, With<Crosshair>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Cam>>,
) {
    for (camera, camera_transform) in camera_query.iter() {
        for focus_trans  in focus_q.iter() {
            for mut style in crosshair_q.iter_mut() {
                let mut dist = 100.;
                if let Ok(target) = target_q.get_single() {
                    dist = target.translation.distance(focus_trans.translation);
                }
    
                let far = focus_trans.translation + focus_trans.forward() * dist;
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

fn show_it(
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

