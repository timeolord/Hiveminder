use bevy::prelude::{Resource, Handle, Image, Plugin, HandleUntyped, ResMut, Res, AssetServer};



#[derive(Resource)]
pub struct TileTextureAtlas {
    pub atlas: Handle<Image>,
    vector: Vec<HandleUntyped>
}

pub struct TextureLoaderPlugin;

impl Plugin for TextureLoaderPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(load_textures);
    }
}

fn load_textures(mut texture_handles: ResMut<TileTextureAtlas>, asset_server: Res<AssetServer>){
    texture_handles.vector = asset_server.load_folder("tiles").expect("Could not load textures");
}