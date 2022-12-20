use std::ops::Range;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{Perlin, NoiseFn};
use ndarray::{Array2};
use self::height::Height;
pub(crate) mod height;
use crate::GameState::{WorldGen, self};
use crate::GameState::Game;

pub struct MapGeneratorPlugin;

trait FromUsize {
    fn from_usize(x: usize, y: usize) -> Self;
}
impl FromUsize for TilemapSize {
    fn from_usize(x: usize, y: usize) -> Self {
        let x = x as u32;
        let y = y as u32;
        Self {x, y}
    }
}
pub struct MapHeight{
    pub max: Height,
    pub min: Height,
}
impl MapHeight{
    pub const fn new(max: usize, min: usize) -> Self{
        Self {
            max: Height{value: max},
            min: Height{value: min}
        }
    } 
    pub fn iter(self: &Self) -> Range<usize>{
        self.min.into()..self.max.into()
    }
}
#[derive(Resource)]
pub struct MapSettings {
    pub size: TilemapSize,
    pub height_limits: MapHeight,
    pub scaling: f64,
    pub tile_size: TilemapTileSize,
    pub heightmap: HeightMap,
    pub texture_handle: Handle<Image>,
}
impl MapSettings {
    pub fn new(size: TilemapSize, height_limits: MapHeight, tile_size: TilemapTileSize, scaling: f64, texture_handle: Handle<Image>) -> Self {
        let heightmap = create_heightmap(0, &size, &height_limits, scaling);
        Self {
            size,
            height_limits,
            scaling,
            tile_size,
            heightmap,
            texture_handle
        }
    }
}

type HeightMap = Array2<Height>;


#[derive(Component)]
pub struct Open;
#[derive(Component)]
pub struct Terrain;
#[derive(Bundle)]
struct GameTileBundle {
    height: Height,
    tile_bundle: TileBundle
}
#[derive(Bundle)]
struct GameTilemapBundle {
    height: Height,
    tilemap_bundle: TilemapBundle
}

pub fn create_tilemap3d(mut commands: Commands, map_settings: Option<Res<MapSettings>>, mut game_state: ResMut<State<GameState>>){
    if map_settings.is_none() {
        return;
    }
    let map_settings = map_settings.unwrap();

    for height in map_settings.height_limits.iter() {
        let height = Height{value: height};
        create_tilemap(&mut commands, height,&map_settings);
    }

    game_state.set(Game).unwrap();
}

fn create_tilemap(commands: &mut Commands, height: Height, map_settings: &MapSettings){
    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(map_settings.size);
    fill_tilemap(commands, tilemap_entity, &mut tile_storage, height,&map_settings);

    let grid_size = map_settings.tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(GameTilemapBundle {
        height: height,
        tilemap_bundle: TilemapBundle {
            grid_size,
            map_type,
            size: map_settings.size,
            storage: tile_storage,
            texture: TilemapTexture::Single(map_settings.texture_handle.clone()),
            tile_size: map_settings.tile_size,
            transform: get_tilemap_center_transform(&map_settings.size, &grid_size, &map_type, 0.0),
            ..Default::default()
        }
    });
}

fn fill_tilemap(commands: &mut Commands, tilemap_entity: Entity, tile_storage: &mut TileStorage, height: Height, map_settings: &MapSettings) {
    for x in 0..map_settings.size.x {
        for y in 0..map_settings.size.y {
            let tile_pos = TilePos { x, y };
            let tile_bundle = GameTileBundle {
                height,
                tile_bundle: TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    color: TileColor(Color::rgb_u8(10, height.value as u8, 10)),
                    visible: TileVisible(false),
                    ..Default::default()
                }
            };
            let tile_entity = if height > map_settings.heightmap[[x as usize, y as usize]] {
                commands
                .spawn((tile_bundle, Open))
                .id()
            } else if height != map_settings.heightmap[[x as usize, y as usize]] {
                commands
                .spawn((tile_bundle, Terrain))
                .id()
            } else {
                commands
                .spawn((tile_bundle, Terrain))
                .id()
            };
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
}

fn create_heightmap(seed: u32, tilemap_size: &TilemapSize, height_limits: &MapHeight, map_scaling: f64) -> HeightMap {
    let noise = Perlin::new(seed);
    let mut noisemap = Array2::from_elem((tilemap_size.x as usize, tilemap_size.y as usize), Height{value: 0});
    let scaling_amount: f64 = (height_limits.max - height_limits.min).into();
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let noise_value = noise.get([x as f64 * map_scaling, y as f64 * map_scaling]);
            let scaled_noise = ((noise_value + 1.0) / 2.0) * scaling_amount;
            noisemap[[x as usize, y as usize]] = Height{value: scaled_noise as usize};
        }
    }
    noisemap
}

fn initalize_resources(mut commands: Commands, asset_server: Res<AssetServer>){
    let max_height = 64;
    let min_height = 0;
    let tile_pixel_length = 16.0;
    let tile_map_size = 32;
    let scaling = 0.1;
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");

    commands.insert_resource(MapSettings::new(
        TilemapSize::from_usize(tile_map_size, tile_map_size),
        MapHeight::new(max_height, min_height),
        TilemapTileSize{x: tile_pixel_length, y: tile_pixel_length },
        scaling,
        texture_handle
    ));
}

impl Plugin for MapGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(WorldGen)
                .with_system(initalize_resources)
        ).add_system_set(
            SystemSet::on_update(WorldGen)
                .with_system(create_tilemap3d)
        );
    }
}