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
mod texture_loader;
mod tiles;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    WorldGen,
    Game,
}

const GAME_TICKS: usize = 20;
pub struct GameTickEvent; // Send a game tick every 20 engine ticks

fn gametick_event_counter(mut tick_counter: Local<usize>, mut gametick_event: EventWriter<GameTickEvent>){
    *tick_counter += 1;
    if *tick_counter >= GAME_TICKS {
        gametick_event.send(GameTickEvent);
        *tick_counter = 0;
    }
}

struct MainGamePlugin;

impl Plugin for MainGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin{
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
        .add_event::<GameTickEvent>()
        .add_system(gametick_event_counter);
    }
}

fn main() {
    App::new()
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: UVec2::new(128, 128),
        })
        .add_plugin(MainGamePlugin)
        .add_state(GameState::MainMenu)
        .add_plugin(TilemapPlugin)
        .add_plugin(MapGeneratorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MainMenuPlugin)
        .run();
}
