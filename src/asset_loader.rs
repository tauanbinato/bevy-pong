use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SpriteAssets {
    pub player: Handle<Image>,
    pub ball: Handle<Image>,
}

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpriteAssets>()
            .add_systems(Startup, load_sprites);
    }
}

fn load_sprites(asset_server: Res<AssetServer>, mut sprite_assets: ResMut<SpriteAssets>) {
    *sprite_assets = SpriteAssets {
        player: asset_server.load("Player.png"),
        ball: asset_server.load("Ball.png"),
    };
}
