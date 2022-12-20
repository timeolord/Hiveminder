use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::tiles::{TileStorage, TileVisible, TilePos};
use crate::map_gen::{Terrain, MapSettings};
use ndarray::{Array3, Array2};
use crate::GameState::Game;
use crate::map_gen::height::Height;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        /* let initalize_display_layer_system = || {
            move |display_height: Res<DisplayHeight>,
            previous_tiles: ResMut<PreviousTiles>,
            map_settings: Res<MapSettings>,
            tiles: Query<(&mut TileVisible, &Height, &TilePos), With<Terrain>>|
            {display_layer(display_height, &mut None, previous_tiles, map_settings, tiles)}}; */

        app.add_system_set(
            SystemSet::on_enter(Game)
                .with_system(initalize_resources)
        )
        .add_system_set(
            SystemSet::on_update(Game)
                .with_system(movement)
                .with_system(display_layer)
        );
    }
}
#[derive(Resource, Clone, Copy)]
pub struct DisplayHeight {
    pub height: Height,
}
#[derive(Resource)]
pub struct PreviousTiles {
    pub tiles: Array3<bool>,
}

#[derive(Component)]
pub struct Visible;

pub fn display_layer(
    display_height: Res<DisplayHeight>,
    mut previous_height: Local<Option<Height>>,
    mut previous_tiles: ResMut<PreviousTiles>,
    map_settings: Res<MapSettings>,
    mut tiles: Query<(&mut TileVisible, &Height, &TilePos), With<Terrain>>) {
        if !display_height.is_changed() {
            return
        }
        if let Some(ref mut prev_height) = *previous_height {
            if display_height.height > *prev_height {
                for (mut tile_visible, height, tile_pos) in tiles.iter_mut() {
                    let index3d = [tile_pos.x as usize, tile_pos.y as usize, height.value as usize];
                    let index2d = [tile_pos.x as usize, tile_pos.y as usize];
                
                    if height.value == display_height.height.value - 1 && map_settings.heightmap[index2d] != display_height.height - 1{
                        tile_visible.0 = false;
                        previous_tiles.tiles[index3d] = false;
                    }
                    else if height.value == display_height.height.value {
                        tile_visible.0 = true;
                        previous_tiles.tiles[index3d] = true;
                    }
                }
            }
            else if display_height.height < *prev_height {
                for (mut tile_visible, height, tile_pos) in tiles.iter_mut() {
                    let index3d = [tile_pos.x as usize, tile_pos.y as usize, height.value as usize];
                
                    if height.value == display_height.height.value + 1{
                        tile_visible.0 = false;
                        previous_tiles.tiles[index3d] = false;
                    }
                    else if height.value == display_height.height.value {
                        tile_visible.0 = true;
                        previous_tiles.tiles[index3d] = true;
                    }
                }
            }
            prev_height.value = display_height.height.value;
        }
        else {
            *previous_height = Some(display_height.height);
            for (mut tile_visible, height, tile_pos) in tiles.iter_mut() {
                let index = [tile_pos.x as usize, tile_pos.y as usize, height.value as usize];
            
                if height.value == display_height.height.value {
                    tile_visible.0 = true;
                    previous_tiles.tiles[index] = true;
                }
            }
        }
}

pub fn initalize_resources(mut commands: Commands, map_settings: Res<MapSettings>){
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(DisplayHeight {height: Height{ value: 0}});
    commands.insert_resource(PreviousTiles {
        tiles: Array3::from_elem(
            [
            map_settings.size.x as usize,
            map_settings.size.y as usize,
            (map_settings.height_limits.max - map_settings.height_limits.min).into()],
            false)
        });
}

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    mut display_height: ResMut<DisplayHeight>,
    map_settings: Res<MapSettings>,
) {
    for mut transform in query.iter_mut() {
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
        if keyboard_input.pressed(KeyCode::Z) {
            display_height.height.value = (display_height.height.value + 1).clamp(map_settings.height_limits.min.into(), map_settings.height_limits.max.into());
        }
        else if keyboard_input.pressed(KeyCode::X) {
            if display_height.height.value != 0 {
                display_height.height.value = (display_height.height.value - 1).clamp(map_settings.height_limits.min.into(), map_settings.height_limits.max.into());
            }
        }
        
        let z = transform.translation.z;
        transform.translation += direction * 250. * time.delta_seconds();
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}