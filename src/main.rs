use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use main_menu::MainMenuPlugin;
use map_gen::MapGeneratorPlugin;
use strum::EnumIter;
use int_enum::IntEnum;
use texture_loader::TextureLoaderPlugin;

mod camera;
mod map_gen;
mod debug;
mod main_menu;
mod texture_loader;
mod tiles;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, EnumIter, IntEnum)]
pub enum GameState {
    InitalizeAssets = 0,
    MainMenu = 1,
    WorldGen = 2,
    Game = 3,
}
pub struct GameTickEvent; // Send a game tick every 20 engine ticks

#[derive(Resource)]
struct GameSpeed(usize);

impl Default for GameSpeed{
    fn default() -> Self {
        Self(20)
    }
}

fn gametick_event_counter(game_speed: Res<GameSpeed>, mut tick_counter: Local<usize>, mut gametick_event: EventWriter<GameTickEvent>){
    *tick_counter += 1;
    if *tick_counter >= game_speed.0 {
        gametick_event.send(GameTickEvent);
        *tick_counter = 0;
    }
}

fn next_game_state(mut game_state: ResMut<State<GameState>>){
    let next_state = GameState::from_int(game_state.current().int_value() + 1).unwrap();
    println!("{:?}", next_state);
    game_state.set(next_state).unwrap();
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
        .init_resource::<GameSpeed>()
        .add_event::<GameTickEvent>()
        .add_system(gametick_event_counter);
    }
}

fn main() {
    App::new()
        .insert_resource(TilemapRenderSettings {
            render_chunk_size: UVec2::new(32, 32),
        })
        .add_plugin(MainGamePlugin)
        .add_state(GameState::from_int(0).unwrap())
        .add_plugin(TilemapPlugin)
        .add_plugin(MapGeneratorPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(MainMenuPlugin)
        .add_plugin(TextureLoaderPlugin)
        .run();
}
