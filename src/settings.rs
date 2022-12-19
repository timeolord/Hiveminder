use bevy_ecs_tilemap::prelude::*;
use crate::map_gen::MapHeight;

//TODO change this all into using resources
const TILE_SQUARE_LENGTH: f32 = 16.0;
pub const TILE_PIXEL_SIZE: TilemapTileSize = TilemapTileSize { x: TILE_SQUARE_LENGTH, y: TILE_SQUARE_LENGTH };
pub const MAX_HEIGHT: u32 = 64;
pub const MIN_HEIGHT: u32 = 0;
pub const MAP_HEIGHT_LIMITS: MapHeight = MapHeight::new(MAX_HEIGHT, MIN_HEIGHT);
pub const MAP_SCALING: f64 = 0.05;
pub const MAP_SIZE: TilemapSize = TilemapSize { x: 64, y: 64 };