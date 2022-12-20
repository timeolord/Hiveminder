use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use main_menu::MainMenuPlugin;
use map_gen::MapGeneratorPlugin;

mod camera;
mod map_gen;
mod debug;
mod main_menu;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    WorldGen,
    Game,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            window: WindowDescriptor {
                width: 1080.0,
                height: 720.0,
                title: String::from(
                    "Rusted Fortress",
                ),
                ..Default::default()
            },
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_state(GameState::WorldGen)
        .add_plugin(TilemapPlugin)
        .add_plugin(MapGeneratorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MainMenuPlugin)
        .run();
}
