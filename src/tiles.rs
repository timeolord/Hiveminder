use std::ops::Range;

use bevy::prelude::{Component, Entity};
use bevy_ecs_tilemap::{prelude::TilemapSize, tiles::{TilePos, TileStorage}};
use itertools::{Itertools, Product};

#[derive(Component, Clone, Copy)]
pub struct GameTilemapSize{
    pub size: TilemapSize,
}
impl GameTilemapSize {
    pub fn new(x: usize, y: usize) -> Self {
        let x = x as u32;
        let y = y as u32;
        Self{size: TilemapSize { x, y }}
    }
}
impl IntoIterator for GameTilemapSize {
    type Item = (usize, usize);
    type IntoIter = Product<Range<usize>, Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        (0..self.size.x as usize).cartesian_product(0..self.size.y as usize)
    }
}
impl From<GameTilemapSize> for TilemapSize {
    fn from(value: GameTilemapSize) -> Self {
        value.size
    }
}
impl From<GameTilemapSize> for [usize; 2] {
    fn from(value: GameTilemapSize) -> Self {
        [value.size.x as usize, value.size.y as usize]
    }
}

trait TupleFrom<T>{
    fn tuple_from(t: T) -> Self;
}
trait TupleInto<U> {
    fn tuple_into(self) -> U;
}
impl<T, U> TupleInto<U> for T
    where
        U: TupleFrom<T>
{
    fn tuple_into(self) -> U { <U as TupleFrom<T>>::tuple_from(self) }
}
impl<T, A, B> TupleFrom<(A, B)> for [T; 2]
    where
        A: Into<T>,
        B: Into<T>,
{
    fn tuple_from(t: (A, B)) -> Self { [t.0.into(), t.1.into()] }
}
impl<T, A, B, C> TupleFrom<(A, B, C)> for [T; 3]
    where
        A: Into<T>,
        B: Into<T>,
        C: Into<T>,
{
    fn tuple_from(t: (A, B, C)) -> Self { [t.0.into(), t.1.into(), t.2.into()] }
}

#[derive(Component, Copy, Clone)]
pub struct GameTilePos {
    pub x: usize,
    pub y: usize,
    pub z: usize
}
impl From<[usize; 3]> for GameTilePos {
    fn from(value: [usize; 3]) -> Self {
        let [x, y, z] = value;
        Self{x, y, z}
    }
}
impl From<[usize; 2]> for GameTilePos {
    fn from(value: [usize; 2]) -> Self {
        let [x, y] = value;
        Self{x, y, z: 0}
    }
}
impl From<(usize, usize)> for GameTilePos {
    fn from(value: (usize, usize)) -> Self {
        let (x, y) = value;
        Self{x, y, z: 0}
    }
}
impl From<(usize, usize, usize)> for GameTilePos {
    fn from(value: (usize, usize, usize)) -> Self {
        let (x, y, z) = value;
        Self{x, y, z}
    }
}
impl From<GameTilePos> for [usize; 2] {
    fn from(value: GameTilePos) -> Self {
        [value.x, value.y]
    }
}
impl From<GameTilePos> for TilePos{
    fn from(value: GameTilePos) -> Self {
        Self {x: value.x as u32, y: value.y as u32}
    }
}

#[derive(Clone, Copy)]
pub struct Game3DSize {
    pub x: usize,
    pub y: usize,
    pub z: usize
}
impl From<Game3DSize> for [usize; 3] {
    fn from(value: Game3DSize) -> Self {
        [value.x, value.y, value.z]
    }
}
pub trait Index2D {
    fn get_2d(self: &Self, coord: (usize, usize)) -> Option<Entity>;
}
impl Index2D for TileStorage{
    fn get_2d(self: &Self, coord: (usize, usize)) -> Option<Entity> {
        self.get(&TilePos{x: coord.0 as u32, y: coord.1 as u32})
    }
}