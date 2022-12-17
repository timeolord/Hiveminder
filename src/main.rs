use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod camera;
mod settings;
mod map_gen;

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    
    let tilemap_size = TilemapSize { x: 64, y: 64 };

    map_gen::create_tilemap3d(commands, texture_handle, tilemap_size, settings::MAP_HEIGHT_LIMITS);
}

/* fn swap_texture_or_hide(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let texture_a = TilemapTexture::Single(asset_server.load("tiles.png"));
        let texture_b = TilemapTexture::Single(asset_server.load("tiles2.png"));
        for (mut tilemap_tex, _) in &mut query {
            if *tilemap_tex == texture_a {
                *tilemap_tex = texture_b.clone();
            } else {
                *tilemap_tex = texture_a.clone();
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::H) {
        for (_, mut visibility) in &mut query {
            if visibility.is_visible {
                visibility.is_visible = false;
            } else {
                visibility.is_visible = true;
            }
        }
    }
} */

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
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(camera::movement)
        .run();
}
