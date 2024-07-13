use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
// use bevy_rapier3d::prelude::*;
use avian3d::prelude::*;

use crate::camera::Cam;
use crate::Target;

pub struct TargetSelectPlugin;
impl Plugin for TargetSelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse_click.run_if(on_event::<MouseButtonInput>()));
     }
}

fn mouse_click(
    mut q_camera: Query<(&Camera, &GlobalTransform), With<Cam>>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    // rapier_context: Res<RapierContext>,
    raycast_q: SpatialQuery,
    old_target_q: Query<Entity, With<Target>>,
    mut commands: Commands
) {
    let Ok((camera, camera_gtransform)) = q_camera.get_single_mut() else {
        return 
    };

    if buttons.just_pressed(MouseButton::Left) {
        let window = q_window.single();
        let Some(cursor_position) = window.cursor_position() else {
            return;
        };
        let Some(ray) = camera.viewport_to_world(camera_gtransform, cursor_position) else {
            return;
        };
    
        // if let Some((entity, _)) = rapier_context.cast_ray(
        //     ray.origin, 
        //     ray.direction.into(),
        //     f32::MAX,
        //     true, 
        //     QueryFilter::default()
        // ) {
        //     if let Ok(old_target) = old_target_q.get_single() {
        //         commands.entity(old_target).remove::<Target>();
        //     }
        //     commands.entity(entity).insert(Target);
        // }

        if let Some(hit) = raycast_q.cast_ray(
            ray.origin, 
            ray.direction.into(),
            f32::MAX,
            true, 
            SpatialQueryFilter::default()
        ) {
            if let Ok(old_target) = old_target_q.get_single() {
                commands.entity(old_target).remove::<Target>();
            }
            commands.entity(hit.entity).insert(Target);
        }
    }

}
