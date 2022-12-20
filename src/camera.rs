use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::tiles::{TileStorage, TileVisible, TilePos};
use crate::map_gen::{Terrain, MapSettings};
use ndarray::Array3;
use crate::GameState::Game;
use crate::map_gen::height::Height;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(Game)
                .with_system(spawn_camera)
        )
        .add_system_set(
            SystemSet::on_update(Game)
                .with_system(movement)
        );
    }
}
#[derive(Resource, Clone, Copy)]
pub struct DisplayHeight {
    pub height: Height,
}

#[derive(Component)]
pub struct Visible;


/* pub fn display_layer(mut commands: Commands, display_height: Res<DisplayHeight>, mut previous_tiles: Local<Array3<bool>>, map_settings: Res<MapSettings>, mut tiles: Query<(Entity, &mut TileVisible, &Height, &TilePos), With<Terrain>>) {
    if !display_height.is_changed() {
        return
    }
    //todo FIX THIS!
    for (_, mut tile_visible, height, tile_pos) in tiles.iter_mut() {
        let index = [tile_pos.x as usize, tile_pos.y as usize, height.value as usize];
        if previous_tiles[index] {
            tile_visible.0 = true;
            previous_tiles[index] = false;
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
} */

pub fn spawn_camera(mut commands: Commands){
    commands.spawn(Camera2dBundle::default());
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
        transform.translation += direction * 16. * time.delta_seconds();
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}