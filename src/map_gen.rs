use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use crate::{settings};
use noise::{Perlin, NoiseFn};

pub struct MapHeight{
    pub max: Height,
    pub min: Height,
    current: Height
}
impl MapHeight{
    pub const fn new(max: u32, min: u32) -> Self{
        Self {
            max: Height(max),
            min: Height(min),
            current: Height(min)
        }
    }
}

impl Iterator for MapHeight {
    type Item = Height;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current.0 += 1;
        if self.current > self.max {
            None
        }
        else {
            Some(result)
        }
    }
}
type NoiseMap = Vec<Vec<Height>>;


#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub struct Height(pub u32);

impl From<Height> for u32 {
    fn from(value: Height) -> Self {
        value.0
    }
}

#[derive(Component)]
pub struct Open;
#[derive(Component)]
pub struct Terrain;
#[derive(Component)]
struct Transparent;

#[derive(Component)]
enum FluidType {
    Water,
    Air
}
struct FluidProperties{
    viscosity: u32
}

#[derive(Bundle)]
struct Fluid {
    fluid_type: FluidType,
}

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

pub fn create_tilemap3d(mut commands: Commands, texture_handle: Handle<Image>, tilemap_size: TilemapSize, height_limits: MapHeight){
    let noisemap = create_heightmap(0, &tilemap_size, &height_limits);

    for height in height_limits {
        create_tilemap(&mut commands, texture_handle.clone(), tilemap_size, height, &noisemap)
    }
}

fn create_tilemap(commands: &mut Commands, texture_handle: Handle<Image>, tilemap_size: TilemapSize, height: Height, noisemap: &NoiseMap){
    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(tilemap_size);
    fill_tilemap(commands, tilemap_size, tilemap_entity, &mut tile_storage, height, &noisemap);

    let tile_size = settings::TILE_PIXEL_SIZE;
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(GameTilemapBundle {
        height: height,
        tilemap_bundle: TilemapBundle {
            grid_size,
            map_type,
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(texture_handle),
            tile_size,
            transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, 0.0),
            ..Default::default()
        }
    });
}

fn fill_tilemap(commands: &mut Commands, tilemap_size: TilemapSize, tilemap_entity: Entity, tile_storage: &mut TileStorage, height: Height, noisemap: &NoiseMap) {
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            let tile_pos = TilePos { x, y };
            let mut tile_bundle = GameTileBundle {
                height,
                tile_bundle: TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    color: TileColor(Color::rgb_u8(10, height.0 as u8, 10)),
                    visible: TileVisible(false),
                    ..Default::default()
                }
            };
            let tile_entity = if height > noisemap[x as usize][y as usize] {
                commands
                .spawn((tile_bundle, Open))
                .id()
            } else if height != noisemap[x as usize][y as usize] {
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

fn create_heightmap(seed: u32, tilemap_size: &TilemapSize, height_limits: &MapHeight) -> NoiseMap{
    let noise = Perlin::new(seed);
    let mut noisemap = vec![vec![Height(0); tilemap_size.x as usize]; tilemap_size.y as usize];
    let scaling_amount = height_limits.max.0 - height_limits.min.0;
    for x in 0..tilemap_size.x {
        for y in 0..tilemap_size.y {
            noisemap[x as usize][y as usize] = Height((((noise.get([x as f64 * settings::MAP_SCALING, y as f64 * settings::MAP_SCALING]) + 1.0) / 2.0) * (scaling_amount - 10) as f64).round() as u32)
        }
    }
    noisemap
}
