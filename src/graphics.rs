use bevy::prelude::*;

pub struct GraphicsPlugin;

#[derive(Resource)]
pub struct CharacterSheet {
    pub handle: Handle<TextureAtlasLayout>,
    pub player_up: [usize; 3],
    pub player_down: [usize; 3],
    pub player_left: [usize; 3],
    pub player_right: [usize; 3],
}

pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct PlayerGraphics {
    pub facing: FacingDirection,
}

#[derive(Component)]
pub struct FrameAnimation {
    pub timer: Timer,
    pub frames: Vec<usize>,
    pub current_frame: usize,
}

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, Self::load_graphics);
        app.add_systems(Update, Self::frame_animation);
            (Self::update_player_graphics);
    }
}

impl GraphicsPlugin {
    fn load_graphics(
        mut commands: Commands,
        assets: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    ) {
        let _image: Handle<Image> = assets.load("janitor_spritesheet.png");
        //let atlas =
            //TextureAtlas::from_grid_with_padding(image, Vec2::splat(16.0), 12, 8, Vec2::splat(2.0));
        
        // the sprite sheet has 7 sprites arranged in a row, and they are all 24px x 24px
        let atlas = TextureAtlasLayout::from_grid(UVec2::splat(48), 3, 4, None, None);
        let atlas_handle = texture_atlases.add(atlas);
       
        let columns = 3;

        commands.insert_resource(CharacterSheet {
            handle: atlas_handle,
            player_down: [1, 2, 3],
            player_left: [columns + 3, columns + 4, columns + 5],
            player_right: [columns * 2 + 3, columns * 2 + 4, columns * 2 + 5],
            player_up: [columns * 3 + 3, columns * 3 + 4, columns * 3 + 5],
        });
    }

    fn update_player_graphics(
        mut sprites_query: Query<(&PlayerGraphics, &mut FrameAnimation), Changed<PlayerGraphics>>,
        characters: Res<CharacterSheet>,
    ) {
        for (graphics, mut animation) in sprites_query.iter_mut() {
            animation.frames = match graphics.facing {
                FacingDirection::Up => characters.player_up.to_vec(),
                FacingDirection::Down => characters.player_down.to_vec(),
                FacingDirection::Left => characters.player_left.to_vec(),
                FacingDirection::Right => characters.player_right.to_vec(),
            }
        }
    }

    fn frame_animation(
        mut sprites_query: Query<(&mut TextureAtlas, &mut FrameAnimation)>,
        time: Res<Time>,
    ) {
        for (mut sprite, mut animation) in sprites_query.iter_mut() {
            animation.timer.tick(time.delta());
            if animation.timer.just_finished() {
                animation.current_frame = (animation.current_frame + 1) % animation.frames.len();
                sprite.index = animation.frames[animation.current_frame];
            }
        }
    }
}
