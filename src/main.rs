use bevy::{prelude::*, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_ecs_tilemap::prelude::*;
use camera::{DisplayHeight, DisplayedTiles};
use map_gen::Height;

mod camera;
mod settings;
mod map_gen;
mod debug;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    
    //settings::MAP_SIZE = TilemapSize { x: 64, y: 64 };

    map_gen::create_tilemap3d(commands, texture_handle, settings::MAP_SIZE, settings::MAP_HEIGHT_LIMITS);
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
        .init_resource::<DisplayHeight>()
        .init_resource::<DisplayedTiles>()
        .add_plugin(TilemapPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(startup)
        .add_startup_system(debug::spawn_debug_text)
        .add_system(debug::update_debug_text)
        .add_system(camera::movement)
        .add_system(camera::display_layer)
        .run();
}
