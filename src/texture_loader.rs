use std::ops::{Index, IndexMut};

use bevy::{prelude::{Resource, Handle, Image, Plugin, ResMut, Res, AssetServer, SystemSet, State, Assets, HandleUntyped}, sprite::{TextureAtlas, TextureAtlasBuilder}};
use bevy_ecs_tilemap::tiles::TileTextureIndex;
use strum::{EnumIter, IntoEnumIterator, Display};
use crate::{GameState::{InitalizeAssets, self}, next_game_state};

#[derive(EnumIter, Display)]
pub enum TileType {
    Grass = 0,
    Stone,
}

impl<T> Index<TileType> for Vec<T>{
    type Output = T;

    fn index(&self, index: TileType) -> &Self::Output {
        &self[index as usize]
    }
}
impl<T> IndexMut<TileType> for Vec<T>{
    fn index_mut(&mut self, index: TileType) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[derive(Resource)]
pub struct TileTextureAtlas {
    pub atlas: Option<TextureAtlas>,
    pub indices: Vec<TileTextureIndex>,
    vector: Vec<Handle<Image>>
}
impl Default for TileTextureAtlas {
    fn default() -> Self {
        Self {
            atlas: None,
            indices: vec![TileTextureIndex(0); TileType::iter().count()],
            vector: Vec::new()
        }
    }
}

pub struct TextureLoaderPlugin;

impl Plugin for TextureLoaderPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .init_resource::<TileTextureAtlas>()
        .add_system_set(SystemSet::on_enter(InitalizeAssets).with_system(load_textures))
        .add_system_set(SystemSet::on_update(InitalizeAssets).with_system(create_texture_atlas));
    }
}

fn load_textures(mut texture_handles: ResMut<TileTextureAtlas>, asset_server: Res<AssetServer>){
    for texture_name in TileType::iter() {
        let texture_path = format!("tiles/{}.png", texture_name.to_string().to_lowercase());
        let texture: Handle<Image> = asset_server.load(texture_path);
        texture_handles.vector.push(texture);
    }
}

fn create_texture_atlas(mut texture_handles: ResMut<TileTextureAtlas>, game_state: ResMut<State<GameState>>, mut textures: ResMut<Assets<Image>>, asset_server: Res<AssetServer>){
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    let untyped_handles: Vec<HandleUntyped> = texture_handles.vector.iter().map(|x| x.clone_untyped()).collect();
    
    match asset_server.get_group_load_state(untyped_handles.iter().map(|x| x.id)) {
        bevy::asset::LoadState::Loaded => {
            for handle in texture_handles.vector.iter() {
                let texture = textures.get(handle).unwrap();
                texture_atlas_builder.add_texture(handle.clone_weak(), texture);
            }
        
            let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

            for texture_name in TileType::iter() {
                let texture_path = format!("tiles/{}.png", texture_name.to_string().to_lowercase());
                let handle: Handle<Image> = asset_server.get_handle(texture_path);
                let index = texture_atlas.get_texture_index(&handle).unwrap();
                texture_handles.indices[texture_name] = TileTextureIndex(index as u32);
            }

            texture_handles.atlas = Some(texture_atlas);
        
            next_game_state(game_state);
        },
        bevy::asset::LoadState::Failed => {
            println!("Failed to load tile textures")
        },
        _ => return,
    }    
}