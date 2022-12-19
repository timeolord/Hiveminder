use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::tiles::{TileStorage, TileVisible, TilePos};
use crate::map_gen::{Height, Open, Terrain};
use crate::settings;
use bitvec::prelude::*;

#[derive(Resource)]
pub struct DisplayHeight {
    pub height: Height,
}
impl Default for DisplayHeight {
    fn default() -> Self {
        Self {
            height: Height(settings::MIN_HEIGHT),
        }
    }
}
#[derive(Resource)]
pub struct DisplayedTiles {
    pub tiles: Vec<Vec<Vec<bool>>>
}
impl Default for DisplayedTiles {
    fn default() -> Self {
        Self {
            tiles: vec![vec![vec![false; settings::MAP_SIZE.x as usize]; settings::MAP_SIZE.y as usize]; (settings::MAX_HEIGHT - settings::MIN_HEIGHT) as usize],
        }
    }
}
#[derive(Component)]
pub struct Visible;



pub fn display_layer(mut commands: Commands, display_height: Res<DisplayHeight>, mut previous_tiles: ResMut<DisplayedTiles>, mut tiles: Query<(Entity, &mut TileVisible, &Height, &TilePos), With<Terrain>>) {
    if !display_height.is_changed() {
        return
    }
    //todo FIX THIS!
    for (_, mut tile_visible, height, tile_pos) in tiles.iter_mut() {
        if previous_tiles.tiles[tile_pos.x as usize][tile_pos.y as usize][height.0 as usize] {
            tile_visible.0 = true;
            previous_tiles.tiles[tile_pos.x as usize][tile_pos.y as usize][height.0 as usize] = false;
        }
    }
    let mut open_positions = vec![vec![true; settings::MAP_SIZE.x as usize]; settings::MAP_SIZE.x as usize];
    
    let mut current_height = display_height.height;
    while open_positions.iter().any(|x| x.iter().any(|y| *y)) {
        for (entity, mut tile_visible, height, tile_pos) in tiles.iter_mut() {
            if current_height == *height && open_positions[tile_pos.x as usize][tile_pos.y as usize] {
                tile_visible.0 = true;
                commands.entity(entity).insert(Visible);
                open_positions[tile_pos.x as usize][tile_pos.y as usize] = false;
            }
        }
        if current_height.0 == (display_height.height.0 as i32 - 8).clamp(settings::MIN_HEIGHT as i32, settings::MAX_HEIGHT as i32) as u32{
            return
        }
        else {
            current_height.0 -= 1;
        }
    }
}

// A simple camera system for moving and zooming the camera.
pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
    mut display_height: ResMut<DisplayHeight>
) {
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

        if keyboard_input.pressed(KeyCode::Z) {
            display_height.height.0 = (display_height.height.0 + 1).clamp(settings::MIN_HEIGHT, settings::MAX_HEIGHT);
        }
        else if keyboard_input.pressed(KeyCode::X) {
            if display_height.height.0 != 0 {
                display_height.height.0 = (display_height.height.0 - 1).clamp(settings::MIN_HEIGHT, settings::MAX_HEIGHT);
            }
        }

        let z = transform.translation.z;
        transform.translation += direction * 16.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}