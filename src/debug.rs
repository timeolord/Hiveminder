use bevy::{prelude::*, diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin}};
use crate::camera::DisplayHeight;
use crate::GameState::Game;

#[derive(Component)]
pub struct DebugText;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_system_set(
            SystemSet::on_enter(Game)
                .with_system(spawn_debug_text)
        )
        .add_system_set(
            SystemSet::on_update(Game)
                .with_system(update_debug_text)
        );
    }
}

const DEBUG_FONT_SIZE: f32 = 20.0;
const DEBUG_FONT: &str = "fonts\\FiraSans-Bold.ttf";

pub fn spawn_debug_text(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load(DEBUG_FONT),
                    font_size: DEBUG_FONT_SIZE,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(DEBUG_FONT),
                font_size: DEBUG_FONT_SIZE,
                color: Color::GOLD,
            }),
            TextSection::new(
                "Layer: ",
                TextStyle {
                    font: asset_server.load(DEBUG_FONT),
                    font_size: DEBUG_FONT_SIZE,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load(DEBUG_FONT),
                font_size: DEBUG_FONT_SIZE,
                color: Color::GOLD,
            }),
        ]),
        DebugText
    ));
}

pub fn update_debug_text(diagnostics: Res<Diagnostics>, display_height: Res<DisplayHeight>, mut query: Query<&mut Text, With<DebugText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.sections[1].value = format!("{value:.2}");
            }
        }
        text.sections[3].value = format!("{}", display_height.height.value);
    }
}