
use bevy::{prelude::*, window::{WindowMode, WindowResolution}};
// use bevy_gltf_components::ComponentsFromGltfPlugin;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
// use bevy_registry_export::*;
use bevy_rapier3d::prelude::*;
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

// ================

fn main() {
    App::new()
    .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
    .add_plugins((
        DefaultPlugins.set(
            WindowPlugin {
                primary_window : Some(Window {
                    // resolution : WindowResolution::new(1400., 900.),
                    mode: WindowMode::BorderlessFullscreen,
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
        target_select::TargetSelectPlugin,
        // WorldInspectorPlugin::new(),
        // ComponentsFromGltfPlugin{legacy_mode: false},
        // ExportRegistryPlugin::default(),
        RapierPhysicsPlugin::<NoUserData>::default(),
        // RapierDebugRenderPlugin::default(),
        ui::UIPlugin,
        HanabiPlugin
    ))
    .init_state::<GameState>()
    .add_event::<GameMessage>()
    .add_systems(Update, check_ready.run_if(in_state(GameState::Setup)))
    .run();
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum GameState{
    #[default]
    Setup,
    Game
}

#[derive(Event, PartialEq)]
pub struct  GameMessage(pub String);

// ---

fn check_ready(
    mut next: ResMut<NextState<GameState>>,
    not_ready_q: Query<Entity, With<NotReady>>
) {
    if not_ready_q.is_empty() {
        next.set(GameState::Game);
    }
} 

