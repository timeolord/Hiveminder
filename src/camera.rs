use bevy::{input::Input, math::Vec3, prelude::*, render::camera::Camera};
use bevy_ecs_tilemap::tiles::{TileVisible, TileStorage};
use crate::GameTickEvent;
use crate::map_gen::{Terrain, MapSettings, Tilemap3D};
use ndarray::Array3;
use crate::GameState::Game;
use crate::map_gen::height::Height;
use crate::tiles::Index2D;

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
    map_settings: Res<MapSettings>,
    tilemap3d: Res<Tilemap3D>,
    tilemaps: Query<&TileStorage>,
    mut tiles: Query<&mut TileVisible, With<Terrain>>) {
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
                        }
                    }
                    if let Some(entity) = previous_layer.get_2d(index){
                        if let Ok(ref mut tile) = tiles.get_mut(entity){
                            if map_settings.heightmap[index].value != display_height.height.value - 1{
                                tile.0 = false;
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
                        }
                    }
                    if let Some(entity) = previous_layer.get_2d(index){
                        if let Ok(ref mut tile) = tiles.get_mut(entity){
                            tile.0 = false;
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
                    }
                }
            }
        }
}


pub fn initalize_resources(mut commands: Commands, map_settings: Res<MapSettings>){
    let map_size: [usize; 3] = map_settings.size.into();
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(DisplayHeight {height: Height{ value: map_settings.height_limits.min.into()}});
    commands.insert_resource(PreviousTiles {
        tiles: Array3::from_elem(
            map_size, false)
        });
}

pub fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
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

pub fn display_height_input(keyboard_input: Res<Input<KeyCode>>, mut display_height: ResMut<DisplayHeight>, map_settings: Res<MapSettings>, gametick_event: EventReader<GameTickEvent>) {
    if gametick_event.is_empty() {
        return
    }
    if keyboard_input.pressed(KeyCode::Z) {
        display_height.height.value = (display_height.height.value + 1).clamp(map_settings.height_limits.min.into(), map_settings.height_limits.max.value - 1);
    }
    else if keyboard_input.pressed(KeyCode::X) {
        if display_height.height.value != 0 {
            display_height.height.value = (display_height.height.value - 1).clamp(map_settings.height_limits.min.into(), map_settings.height_limits.max.value - 1);
        }
    }
}