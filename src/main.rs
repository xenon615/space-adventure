
use bevy::{prelude::*, window::{WindowMode, WindowResolution}};
use std::time::Duration;
use bevy::time::common_conditions::on_timer;
// use bevy_gltf_components::ComponentsFromGltfPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_registry_export::*;
use avian3d::prelude::*;
use bevy_hanabi::prelude::*;
mod camera;
mod env;
mod drone;
mod ui;
mod effects;
mod asteroids;
mod docks;
mod missile;
mod laser;
mod crosshair;
mod target_select;
// ===============

#[derive(Component)]
pub struct Target;
#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct LifeTime(pub f32);

#[derive(Component)]
pub struct NotReady;

#[derive(Component)]
pub struct ForDestroy;


// ================

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
    .add_plugins((
        DefaultPlugins.set(
            WindowPlugin {
                primary_window : Some(Window {
                    resolution : WindowResolution::new(1400., 900.),
                    title: "Space Adventure".into(),
                    // mode: WindowMode::BorderlessFullscreen,
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }
        ),
        camera::CameraPlugin,
        env::EnvPlugin,
        drone::DronePlugin,
        missile::MissilePlugin,
        laser::LaserPlugin,
        asteroids::AsteroidsPlugin,
        docks::DocksPlugin,
        crosshair::CrosshairPlugin,
        target_select::TargetSelectPlugin,


        // WorldInspectorPlugin::new(),
        // ComponentsFromGltfPlugin{legacy_mode: false},
        // ExportRegistryPlugin::default(),
        // RapierPhysicsPlugin::<NoUserData>::default(),
        PhysicsPlugins::default(),
        // RapierDebugRenderPlugin::default(),
        ui::UIPlugin,
        HanabiPlugin
    ))
    .init_state::<GameState>()
    .add_systems(Update, check_ready.run_if(in_state(GameState::Setup)))
    .add_systems(Update, (cleanup, overtime).run_if(on_timer(Duration::from_secs(1))))
    .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState{
    #[default]
    Setup,
    Game
}

// ---

fn check_ready(
    mut next: ResMut<NextState<GameState>>,
    not_ready_q: Query<Entity, With<NotReady>>
) {
    if not_ready_q.is_empty() {
        next.set(GameState::Game);
    }
} 

// ---

fn cleanup(
    mut commands: Commands,
    delete_q: Query<Entity, With<ForDestroy>>,
) {
    for e in delete_q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

// ---

fn overtime(
    mut commands: Commands,
    delete_q: Query<(Entity, &LifeTime)>,
    time: Res<Time>,
) {
    for (e,  lifetime) in delete_q.iter() {
        if lifetime.0 < time.elapsed_seconds() {
            commands.entity(e).despawn_recursive();
        }
    }
}