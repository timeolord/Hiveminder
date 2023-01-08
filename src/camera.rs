use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::prelude::*;
use bevy_ecs_tilemap::tiles::{TileVisible, TileStorage, TileColor};
use crate::map_gen::{Terrain, MapSettings, Tilemap3D};
use crate::GameState::Game;
use crate::map_gen::height::Height;
use crate::texture_loader::TileTextureAtlas;
use crate::tiles::{Index2D, GameTilePos};
use crate::texture_loader::TileType::Shadow;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(Game)
                .with_system(initalize_resources)
        )
        .add_system_set(
            SystemSet::on_update(Game)
                .with_system(movement)
                .with_system(display_layer)
                .with_system(display_height_input)
                .with_system(create_shadows)
                //.with_system(add_shadows)
        );
    }
}
#[derive(Resource, Clone, Copy)]
pub struct DisplayHeight {
    pub height: Height,
}
#[derive(Component)]
pub struct Visible;

pub fn display_layer(
    mut commands: Commands,
    display_height: Res<DisplayHeight>,
    mut previous_height: Local<Option<Height>>,
    map_settings: Res<MapSettings>,
    tilemap3d: Res<Tilemap3D>,
    tilemaps: Query<&TileStorage>,
    mut tiles: Query<&mut TileVisible, With<Terrain>>)
    {
    if !display_height.is_changed() {
        return
    }
    if let Some(ref mut prev_height) = *previous_height {
        if display_height.height > *prev_height {
            let layers = [tilemap3d.layers[display_height.height.value], tilemap3d.layers[display_height.height.value - 1]];
            let [current_layer, previous_layer] = tilemaps.many(layers);

            for index in map_settings.layer_size {
                if let Some(entity) = current_layer.get_2d(index){
                    if let Ok(ref mut tile) = tiles.get_mut(entity){
                        tile.0 = true;
                        commands.entity(entity).insert(Visible);
                    }
                }
                if let Some(entity) = previous_layer.get_2d(index){
                    if let Ok(ref mut tile) = tiles.get_mut(entity){
                        if map_settings.heightmap[index].value != display_height.height.value - 1{
                            tile.0 = false;
                            commands.entity(entity).remove::<Visible>();
                        }
                    }
                }
            }
        }
        else if display_height.height < *prev_height {
            let layers = [tilemap3d.layers[display_height.height.value], tilemap3d.layers[display_height.height.value + 1]];
            let [current_layer, previous_layer] = tilemaps.many(layers);

            for index in map_settings.layer_size {
                if let Some(entity) = current_layer.get_2d(index){
                    if let Ok(ref mut tile) = tiles.get_mut(entity){
                        tile.0 = true;
                        commands.entity(entity).insert(Visible);
                    }
                }
                if let Some(entity) = previous_layer.get_2d(index){
                    if let Ok(ref mut tile) = tiles.get_mut(entity){
                        tile.0 = false;
                        commands.entity(entity).remove::<Visible>();
                    }
                }
            }
        }
        prev_height.value = display_height.height.value;
    }
    else {
        *previous_height = Some(display_height.height);
        let layer = tilemaps.get(tilemap3d.layers[display_height.height.value]).unwrap();

        for index in map_settings.layer_size {
            if let Some(entity) = layer.get_2d(index){
                if let Ok(ref mut tile) = tiles.get_mut(entity){
                    tile.0 = true;
                    commands.entity(entity).insert(Visible);
                }
            }
        }
    }
}


pub fn initalize_resources(mut commands: Commands, map_settings: Res<MapSettings>){
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(DisplayHeight {height: Height{ value: map_settings.height_limits.min.into()}});
}

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,)
    {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Q) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::E) {
            ortho.scale -= 0.1;
        }

        let z = transform.translation.z;
        transform.translation += direction * 250. * time.delta_seconds() * ortho.scale;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}

pub fn display_height_input(keyboard_input: Res<Input<KeyCode>>, mut display_height: ResMut<DisplayHeight>, map_settings: Res<MapSettings>) {
    /* if gametick_event.is_empty() {
        return
    } */
    if keyboard_input.pressed(KeyCode::Z) {
        display_height.height.value = (display_height.height.value + 1).clamp(map_settings.height_limits.min.into(), map_settings.height_limits.max.value - 1);
    }
    else if keyboard_input.pressed(KeyCode::X) {
        if display_height.height.value != 0 {
            display_height.height.value = (display_height.height.value - 1).clamp(map_settings.height_limits.min.into(), map_settings.height_limits.max.value - 1);
        }
    }
}

pub fn add_shadows(
    display_height: Res<DisplayHeight>,
    mut tiles: Query<(&Height, &mut TileColor), With<Visible>>)
    {
    if !display_height.is_changed() {
        return
    }
    for (height, mut color) in tiles.iter_mut(){
        let height_difference = (255 - (display_height.height - *height).value) as u8;

        //color.0 = Color::BLACK;
        //color.0 = Color::rgb_u8(height_difference, height_difference, height_difference);
        //color = TileColor(Color::rgba_u8(255, height.value as u8, 255, 255));
    }
}

#[derive(Component)]
pub struct ShadowMap;

pub fn create_shadows(
    mut commands: Commands,
    map_settings: Res<MapSettings>,
    texture_atlas: Res<TileTextureAtlas>,
    display_height: Res<DisplayHeight>,
    previous_shadow_map: Query<Entity, With<ShadowMap>>,
    tiles: Query<&GameTilePos, (With<Visible>, Without<ShadowMap>)>)
    {
    if !display_height.is_changed() {
        return
    }

    for previous_entity in previous_shadow_map.iter(){
        commands.entity(previous_entity).despawn();
    }

    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_settings.layer_size.into());

    let grid_size = map_settings.tile_size.into();
    let map_type = TilemapType::default();

    for position in tiles.iter(){
        let height_difference = (display_height.height.value.saturating_sub(position.z) as u8).saturating_mul(4);
        let tile_2d_pos = (*position).into();
        let tile_bundle =
            TileBundle {
                texture_index: texture_atlas.indices[Shadow],
                position: tile_2d_pos,
                tilemap_id: TilemapId(tilemap_entity),
                color: TileColor(Color::rgba_u8(0, 0, 0, height_difference)),
                visible: TileVisible(true),
                ..Default::default()
            };
        let tile_entity = commands.spawn((tile_bundle, ShadowMap)).id();
        tile_storage.set(&tile_2d_pos, tile_entity);
    }

    commands.entity(tilemap_entity).insert((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_settings.layer_size.into(),
            storage: tile_storage,
            texture: TilemapTexture::Single(map_settings.texture_handle.clone()),
            tile_size: map_settings.tile_size,
            transform: get_tilemap_center_transform(&map_settings.layer_size.into(), &grid_size, &map_type, 1.0),
            ..Default::default()
        }, ShadowMap));
}