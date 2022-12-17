use bevy_ecs_tilemap::prelude::*;
use crate::map_gen::MapHeight;

const TILE_SQUARE_LENGTH: f32 = 16.0;
pub const TILE_PIXEL_SIZE: TilemapTileSize = TilemapTileSize { x: TILE_SQUARE_LENGTH, y: TILE_SQUARE_LENGTH };
const MAX_HEIGHT: u32 = 64;
const MIN_HEIGHT: u32 = 0;
pub const MAP_HEIGHT_LIMITS: MapHeight = MapHeight::new(MAX_HEIGHT, MIN_HEIGHT);
pub const MAP_SCALING: f64 = 0.05;