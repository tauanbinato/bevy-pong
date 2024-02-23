use crate::{
    asset_loader::SpriteAssets,
    movement::{Acceleration, MovingObjectBundle, Velocity},
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(50.0, 0.0, 0.0);

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player);
    }
}

fn spawn_player(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands.spawn((
        MovingObjectBundle {
            collider: Collider::cuboid(10.0, 60.0),
            velocity: Velocity::new(Vec3::ZERO),
            acceleration: Acceleration::new(Vec3::ZERO),
            model: SpriteBundle {
                texture: sprite_assets.player.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
        },
        Player,
    ));
}
