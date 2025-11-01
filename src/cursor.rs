////Not included in final push
// use bevy::prelude::*;

// pub fn draw_cursor(
//     camera_query: Single<(&Camera, &GlobalTransform)>,
//     windows: Query<&Window>,
//     mut gizmos: Gizmos,
// ) {
//     let (camera, camera_transform) = *camera_query;

//     let Ok(window) = windows.get_single() else {
//         return;
//     };

//     let Some(cursor_position) = window.cursor_position() else {
//         return;
//     };

//     // Calculate a world position based on the cursor's position.
//     let Ok(point) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
//         return;
//     };

//     gizmos.circle_2d(point, 10., WHITE);
// }

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2d);
// }
