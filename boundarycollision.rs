//Not made into final push 

const LEFT_WALL: f32 = -450.;
const RIGHT_WALL: f32 = 450.;

const BOTTOM_WALL: f32 = -300.;
const TOP_WALL: f32 = 300.;


struct WallBundle {
    transform: Transform,
    collider: Collider,
}

//Would go in main i think
    // commands.spawn(WallBundle::new(WallLocation::Left));
    // commands.spawn(WallBundle::new(WallLocation::Right));
    // commands.spawn(WallBundle::new(WallLocation::Bottom));
    // commands.spawn(WallBundle::new(WallLocation::Top));


impl WallLocation {
    fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(LEFT_WALL, 0.),
            WallLocation::Right => Vec2::new(RIGHT_WALL, 0.),
            WallLocation::Bottom => Vec2::new(0., BOTTOM_WALL),
            WallLocation::Top => Vec2::new(0., TOP_WALL),
        }
    }

    fn size(&self) -> Vec2 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, arena_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(arena_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}

// use bevy::{prelude::*, render::camera::ScalingMode, transform::{self, commands}, window::PrimaryWindow};
// use bevy_ecs_tiled::{TiledMapHandle, TiledMapPlugin};
// use bevy_ecs_tilemap::prelude::*;
// use bevy_rapier2d::{prelude::{Collider, Sensor}, rapier::prelude::CollisionEvent};
// use bevy_tweening::Tween;
// // use r#move::{derive_z_from_y_after_move, move_camera, move_player};
// use std::time::Duration;

// #[derive(Debug, PartialEq, Eq, Copy, Clone)]
// enum Collision {
//     Left,
//     Right,
//     Top,
//     Bottom,
// }

// fn check_for_collisions(
//     mut commands: Commands,
//     mut sprite: SpriteBundle,
//     mut tasks: Vec<SpriteBundle>,
//     mut player_query: Query<(Entity, &Transform), With<Task>,
//     mut collider_query: Query<&Transform, With<SpriteBundle>,
    
// ) {

//     let (sprite) = player_query.into_inner();

//     for collider_entity in collider_query
//     {
//         let collision = player_collision(sprite.transform);
//         if let Some(collision) = collision {
//             collision_events.send_default();
        
//     };
// }
// }

// fn player_collision(player: Transform,) -> Option<Collision> {
//     if !player.intersects(&bounding_box) {
//         return None;
//     }

//     let closest = bounding_box.closest_point(player.center());
//     let offset = player.center() - closest;
//     let side = if offset.x.abs() > offset.y.abs() {
//         if offset.x < 0. {
//             Collision::Left
//         } else {
//             Collision::Right
//         }
//     } else if offset.y > 0. {
//         Collision::Top
//     } else {
//         Collision::Bottom
//     };

//     Some(side)
// };