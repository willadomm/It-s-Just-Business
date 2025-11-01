use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use crate::GameState;
use crate::text::ButtonPressState;

pub struct GameAudioPlugin;

#[derive(Resource)]
pub struct Background;

#[derive(Resource)]
pub struct ButtonPress;


#[derive(Event, Resource)]
pub struct ButtonPressTriggered;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        
        app.add_audio_channel::<Background>();
        app.add_audio_channel::<ButtonPress>();
        app.add_systems(OnEnter(GameState::Playing), play_bgm);
        //app.add_systems(OnExit(GameState::Menu), play_button_press);
        app.add_systems(Update, play_button_press);
    }
}



pub fn play_bgm(background: Res<AudioChannel<Background>>, asset_server: Res<AssetServer>, _audio: Res<Audio>) {
    background.set_volume(0.05);
    background
    .play(asset_server.load("retroindiejosh_50s-bit.ogg")).looped();

}


pub fn play_button_press(
    mut button_press_event_writer: EventWriter<ButtonPressTriggered>,
    button_press: Res<AudioChannel<ButtonPress>>,
    asset_server: Res<AssetServer>,
    _audio: Res<Audio>,
    mut button_press_state: ResMut<ButtonPressState>
) {
    if button_press_state.triggered {
        button_press.set_volume(0.15);
        button_press.play(asset_server.load("Menu_In.ogg"));
        
        button_press_event_writer.send(ButtonPressTriggered);
        button_press_state.triggered = false;
    }

}
