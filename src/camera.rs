use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};


use bevy::core_pipeline::bloom::BloomSettings;
use bevy::prelude::*;
use bevy::core_pipeline::Skybox;
// use bevy_rapier3d::prelude::*;
// use bevy::transform::TransformSystem;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup); 
        // app.add_systems(PostUpdate, 
        //     follow
        //     .run_if(not(in_state(CamViewState::Free)))
        //     .after(PhysicsSet::SyncBackend)
        //     .before(TransformSystem::TransformPropagate)
        // );
        app.add_systems(Update, follow.run_if(not(in_state(CamViewState::Free))));

        app.add_systems(Update, switch_state);

        app.add_systems(OnEnter(CamViewState::Free), enter_free);
        app.add_systems(OnExit(CamViewState::Free), exit_free);

        app.add_plugins(PanOrbitCameraPlugin);
        app.init_state::<CamViewState>();
    }
} 

#[derive(Component)]
pub struct Cam;

#[derive(Component)]
pub struct Focus;

#[derive(Component, Debug)]
pub struct CamBias(Vec3, Vec3);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum CamViewState {
    #[default]
    Third,
    Back,
    Right,
    Left,
    Top,
    Free
}

// ---

fn setup (
    mut commands : Commands,
    assets: ResMut<AssetServer>
) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(120., 15., 100.),
            projection: PerspectiveProjection {
                fov: 60.0_f32.to_radians(),
                ..default()
            }.into(),
            camera: Camera {
                hdr: true,
                ..default()
            },
            ..default()
        },
        Skybox{
            image: assets.load("skyboxes/cubemap.ktx2"),
            brightness: 1500.
        }, 
        Cam,
        Name::new("Camera"),
        CamBias(Vec3::new(0., 10., -45.), Vec3::new(0., 10., 0.)),
        BloomSettings::NATURAL,
        PanOrbitCamera {
            enabled: false,
            ..default()
        }
    ));
}

// ---

fn follow (
    q_l: Query<&Transform , With<Focus>>,
    mut q_c: Query<(&mut Transform, &CamBias), (With<Cam>, Without<Focus>)>,
    time: Res<Time>,
) {
    if let Ok(target) = q_l.get_single() {
        if let Ok((mut cam, bias)) = q_c.get_single_mut() {
            let desired = target.translation + target.right() * bias.0.x + target.up() * bias.0.y + target.forward() * bias.0.z;
            cam.translation = cam.translation.lerp(desired, time.delta_seconds() * 1.);
            let look_at = target.translation + target.right() * bias.1.x + target.up() * bias.1.y + target.forward() * bias.1.z;
            cam.rotation = cam.rotation.slerp(cam.looking_at(look_at, Vec3::Y).rotation, time.delta_seconds() * 5.);
        }
    }
}

// ---

fn switch_state(
    mut next: ResMut<NextState<CamViewState>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut cam_q: Query<&mut CamBias, With<Cam>>
) {

    if keys.any_just_pressed([KeyCode::Digit1, KeyCode::Digit2, KeyCode::Digit3, KeyCode::Digit4, KeyCode::Digit5, KeyCode::Digit6]) {
        let Ok(mut bias) = cam_q.get_single_mut() else {
            return;
        };
        for b in keys.get_just_pressed() {
            match b {
                KeyCode::Digit1 => {
                    bias.0 = Vec3::new(0., 10., -45.);
                    bias.1 = Vec3::new(0., 10., -5.);
                    next.set(CamViewState::Third);
                },
                KeyCode::Digit2 => {
                    bias.0 = Vec3::new(0., 0., -30.);
                    bias.1 =  Vec3::splat(0.);
                    next.set(CamViewState::Back);
                },
                KeyCode::Digit3 => {
                    bias.0 = Vec3::new(0., 30., 0.);
                    bias.1 = Vec3::splat(0.);
                    next.set(CamViewState::Top);
                },

                KeyCode::Digit4 => {
                    bias.0 = Vec3::new(-30., 0., 0.);
                    bias.1 = Vec3::splat(0.);
                    next.set(CamViewState::Left);
                },
                KeyCode::Digit5 => {
                    bias.0 = Vec3::new(30., 0., 0.);
                    bias.1 = Vec3::splat(0.);
                    next.set(CamViewState::Right);
                },

                KeyCode::Digit6 => {
                    next.set(CamViewState::Free);
                },

             _ => ()   
            }
        }
    }
}

// ---

fn enter_free(
    focus_q: Query<&Transform, With<Focus>>,
    mut pan_orbit_query: Query<&mut PanOrbitCamera>
) {
    if let Ok(mut poc)  = pan_orbit_query.get_single_mut() {
        if let Ok(ft) = focus_q.get_single() {
            poc.target_focus = ft.translation;
        }
        poc.enabled = true;
        poc.force_update = true;
    }
}

// ---

fn exit_free(
    mut pan_orbit_query: Query<&mut PanOrbitCamera>
) {
    if let Ok(mut poc)  = pan_orbit_query.get_single_mut() {
        poc.enabled = false;
        poc.force_update = true;
    }
}
