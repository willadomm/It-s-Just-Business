use bevy::{prelude::*, render::camera::ScalingMode, transform::commands, window::PrimaryWindow};
use bevy_ecs_tiled::{TiledMapHandle, TiledMapPlugin};
use bevy_ecs_tilemap::prelude::*;
use bevy_tweening::Tween;
use std::time::Duration;

use bevy::math::vec3;
use bevy_tweening::*;
use lens::TransformPositionLens;

mod cursor;
mod mainmenu;
mod text;

use crate::mainmenu::MenuPlugin;


use bevy_spritesheet_animation::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::prelude::Window;
use bevy_text_popup::TextPopupPlugin;
use text::handle_next_popup;
use text::welcome_setup;
use text::game_ui;
use text::update_time;

mod audio;
use audio::GameAudioPlugin;
use bevy_kira_audio::AudioPlugin;
use audio::play_button_press;
use audio::ButtonPressTriggered;




#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Settings,
    Playing,
    Paused,
    Exit,
}

#[derive(Component)]
struct Collider;

#[derive(Resource)]
struct RootEntity(Entity);

fn despawn_state(mut commands: Commands, root: Res<RootEntity>) {
    commands.entity(root.0).despawn_recursive();
}

fn quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
}

fn create_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct MapInfo {
    map_width: f32,
    map_height: f32,
}

#[derive(Bundle)]
struct Player {
    position: Position,
    sprite: SpriteBundle,
    speed: Speed,
}

#[derive(Component)]
struct Position {
    position: Vec<f32>,
}

#[derive(Component)]
struct Speed {
    speed: i32,
}

// #[derive(Component)]
struct MyCameraMarker;

fn main() {
    // Create a new application.
    App::default()
    .add_plugins(DefaultPlugins
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "It's Just Business".into(),
                resolution: (1280.0, 720.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        })
        .set(ImagePlugin::default_nearest())
    )
    .add_plugins((
        bevy_tweening::TweeningPlugin,
        TilemapPlugin,
        MenuPlugin,
        SpritesheetAnimationPlugin,
        TiledMapPlugin::default(),
        TextPopupPlugin,
        GameAudioPlugin,
        AudioPlugin,
    ))
    //.add_plugins(EguiPlugin)
    .init_state::<GameState>()
    .add_event::<ButtonPressTriggered>()
    .insert_resource(MapInfo {
        map_width: 30.0,
        map_height: 20.0,
    })
    .add_systems(Startup, (
        spawn_entity,
        setup,
        spawn_camera,
        scale_tilemap_to_screen,
        welcome_setup,
        //game_ui,
    ))    
    //.add_systems(Update, ui_example_system)
    .add_systems(
        Update,
        (
            keyboard_input,
            play_button_press,
            handle_next_popup.run_if(in_state(GameState::Playing)),
            //game_ui.run_if(in_state(GameState::Playing)),
            update_time.run_if(in_state(GameState::Playing)),
            
        )
    )
    .add_systems(OnEnter(GameState::Playing), game_ui)
    .add_systems(OnEnter(GameState::Playing), update_time)
    .run();
    
}

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}


// Loads tilemap and janitor sprite.
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load the tilemap
    commands.spawn((
        TiledMapHandle(asset_server.load("tilemap_level1.tmx")),
        Transform::default(),
        GlobalTransform::default(),
    ));

    // The tilemap is 20x30 tiles, each 24x24 pixels.
    let map_tile_width = 30.0;
    let map_tile_height = 20.0;
    let tile_size = 24.0;

    let map_width = map_tile_width * tile_size;
    let map_height = map_tile_height * tile_size;

    // Store the map info in a resource
    commands.insert_resource(MapInfo {
        map_width,
        map_height,
    });
    

    info!("Setup complete. Map size: {}x{}", map_width, map_height);
}

fn spawn_camera(mut commands: Commands) {
    // Spawn a 2D camera
    let mut our_camera = Camera2dBundle::default();
    our_camera.transform = Transform::from_xyz(350.0, 240.0, 1.0);
    our_camera.projection.scaling_mode = ScalingMode::FixedVertical(500.0);

    commands.spawn(our_camera);
}

fn scale_tilemap_to_screen(
    mut query: Query<&mut Transform, With<TiledMapHandle>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    map_info: Res<MapInfo>,
) {
    let window = primary_window.single();
    let window_width = window.width();
    let window_height = window.height();

    // Calculate the scale to fit the map to the screen
    let scale_x = window_width / map_info.map_width;
    let scale_y = window_height / map_info.map_height;

    // Choose the smaller scale to ensure the entire map fits within the window
    let scale = scale_x.min(scale_y);

    for mut transform in query.iter_mut() {
        // Apply scaling
        transform.scale = Vec3::splat(scale);

        // Center the map on the screen
        transform.translation = Vec3::new(
            (window_width - map_info.map_width * scale) / 2.0,
            (window_height - map_info.map_height * scale) / 2.0,
            0.0,
        );
    }

    info!(
        "Window size: {}x{}, Map size: {}x{}, Scale: {}",
        window_width, window_height, map_info.map_width, map_info.map_height, scale
    );
}

#[derive(Resource)]
struct PosVar {
    pos_vec: Vec3,
    id: Entity,
    timer: Timer,
    in_anim: bool,
    last_direction: Option<Vec3>,
    
}
fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut local: ResMut<PosVar>,
    mut commands: Commands,
    time: Res<Time>,
    library: Res<SpritesheetLibrary>,
    mut characters: Query<(
        Entity,
        &mut Transform,
        &mut Sprite,
        &mut SpritesheetAnimation,
        
    )>,
) {
    for (entity, mut transform, mut sprite, mut animation) in &mut characters {
        local.timer.tick(time.delta());
        if local.timer.just_finished() {
            local.in_anim = false;
        }

        if !local.in_anim {
            let mut new_animation_id = None;
            let mut direction = None;
            let mut target_position = local.pos_vec;

            if keys.pressed(KeyCode::ArrowRight) {
                new_animation_id = library.animation_with_name("rightwalk");
                direction = Some(vec3(27., 0., 0.));
                local.last_direction = Some(direction.unwrap());

            } else if keys.pressed(KeyCode::ArrowLeft) {
                new_animation_id = library.animation_with_name("leftwalk");
                direction = Some(vec3(-27., 0., 0.));
                local.last_direction = Some(direction.unwrap());

            } else if keys.pressed(KeyCode::ArrowDown) {
                new_animation_id = library.animation_with_name("frontwalk");
                direction = Some(vec3(0., -27., 0.));
                local.last_direction = Some(direction.unwrap());

            } else if keys.pressed(KeyCode::ArrowUp) {
                new_animation_id = library.animation_with_name("upwardwalk");
                direction = Some(vec3(0., 27., 0.));
                local.last_direction = Some(direction.unwrap());

            }
            if !keys.pressed(KeyCode::ArrowRight)
                && !keys.pressed(KeyCode::ArrowLeft)
                && !keys.pressed(KeyCode::ArrowDown)
                && !keys.pressed(KeyCode::ArrowUp)
            {
                new_animation_id = Some(match local.last_direction {
                    Some(dir) if dir == vec3(27., 0., 0.) => library.animation_with_name("rightidle").unwrap(),
                    Some(dir) if dir == vec3(-27., 0., 0.) => library.animation_with_name("leftidle").unwrap(),
                    Some(dir) if dir == vec3(0., -27., 0.) => library.animation_with_name("frontidle").unwrap(),
                    Some(dir) if dir == vec3(0., 27., 0.) => library.animation_with_name("upwardidle").unwrap(),
                    _ => library.animation_with_name("frontidle").unwrap(),
                });
            }
            
            

            if let Some(animation_id) = new_animation_id {
                if animation.animation_id != animation_id {
                    animation.animation_id = animation_id;
                    animation.reset();
                }

                if let Some(dir) = direction {
                    target_position = local.pos_vec + dir;

                    let tween = Tween::new(
                        EaseFunction::QuadraticInOut,
                        Duration::from_millis(250),
                        TransformPositionLens {
                            start: local.pos_vec,
                            end: target_position,
                        },
                    );

                    commands
                        .entity(local.id)
                        .remove::<Animator<Transform>>()
                        .insert(Animator::new(tween));

                    local.pos_vec = target_position;
                    local.timer.reset();
                    local.in_anim = true;
                }
            }
        }
    }
}



fn spawn_entity(
    mut commands: Commands,
     asset_server: Res<AssetServer>,
     mut library: ResMut<SpritesheetLibrary>,
     mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) {
    let tween = Tween::new(
        EaseFunction::QuadraticInOut,
        Duration::from_secs(1),
        TransformPositionLens {
            start: Vec3::new(360.0, 410.0, 1.0),
            end: Vec3::new(360.0, 410.0, 1.0),
        },
    )
    .with_repeat_count(RepeatCount::Finite(2))
    .with_repeat_strategy(RepeatStrategy::MirroredRepeat);

    let janitor_texture: Handle<Image> = asset_server.load("janitor_spritesheet.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(32, 48),
        3,
        4,
        None,
        None,
    ));

    //Left idle
    let leftidle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 4).row_partial(2, 0..=0));

    });
    let leftidle_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(leftidle_clip_id.into());
            });
    library.name_animation(leftidle_anim_id, "leftidle").unwrap();
   
    //Right idle
    let rightidle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 4).row_partial(3, 0..=0));

    });
    let rightidle_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(rightidle_clip_id.into());
            });
    library.name_animation(rightidle_anim_id, "rightidle").unwrap();         
    //Front idle
    let frontidle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 4).row_partial(0, 0..=0));

    });
    let frontidle_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(frontidle_clip_id.into());
            });
    library.name_animation(frontidle_anim_id, "frontidle").unwrap();

    //Upward idle
    let upwardidle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 4).row_partial(1, 0..=0));

    });
    let upwardidle_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(upwardidle_clip_id.into());
            });
    library.name_animation(upwardidle_anim_id, "upwardidle").unwrap();      

    //Left walking direction
    let leftwalk_clip_id = library.new_clip(|clip| {
        //clip.push_frame_indices(Spritesheet::new(3, 4).horizontal_strip(1, 3, 3));
        clip.push_frame_indices(Spritesheet::new(3, 4).row_partial(2, 1..=2));

    });
    let leftwalk_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(leftwalk_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);
            });
    library.name_animation(leftwalk_anim_id, "leftwalk").unwrap();


    //Right walking direction
    let rightwalk_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 4).row_partial(3, 1..=2));

    });
    let rightwalk_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(rightwalk_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);  
        });
    library.name_animation(rightwalk_anim_id, "rightwalk").unwrap();

    //Front walking direction
    let frontwalk_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 4).row_partial(0, 1..=2));
    });
    let frontwalk_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(frontwalk_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);  
    
    });
    library.name_animation(frontwalk_anim_id, "frontwalk").unwrap();

    //Upward walking direction
    let upwardwalk_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(3, 4).row_partial(1, 1..=2));

    });
    let upwardwalk_anim_id = library.new_animation(|animation| {
        animation
            .add_stage(upwardwalk_clip_id.into())
            .set_repeat(AnimationRepeat::Loop);   
          
        });
    library.name_animation(upwardwalk_anim_id, "upwardwalk").unwrap();

 
    let id = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: bevy::color::Color::WHITE,
                    custom_size: Some(Vec2::new(21., 32.)),
                    ..default()
                },
                texture: janitor_texture,
                transform: Transform {
                    translation: Vec3::new(360.0, 410.0, 1.0),
                    ..Default::default()
                },
                ..default()
            },
            TextureAtlas {
                layout,
                ..default()
            },
            SpritesheetAnimation::from_id(frontidle_anim_id),
            Animator::new(tween),

        ))
        .id();
    commands.insert_resource(PosVar {
        in_anim: false,
        pos_vec: Vec3::new(360., 410., 1.),
        id: id,
        timer: Timer::from_seconds(0.25, TimerMode::Once),
        last_direction: None,
    });
}

fn spawn_task(mut commands: Commands, asset_server: Res<AssetServer>) {
    let planttexture: Handle<Image> = asset_server.load("plant_asset1.png");
    let mut plant_object = SpriteBundle::default();

    plant_object.texture = planttexture;

    plant_object.transform = Transform::from_xyz(100.0, 410.0, 1.0);
    commands.spawn(plant_object);
}
