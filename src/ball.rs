use crate::{asset_loader::SpriteAssets, movement::MovingObjectRigidBodyDynamicBundle};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Debug)]
pub struct Ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ball);
    }
}

fn spawn_ball(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands.spawn((
        MovingObjectRigidBodyDynamicBundle {
            model: SpriteBundle {
                texture: sprite_assets.ball.clone(), // Assuming you have a ball texture in your sprite assets
                transform: Transform::from_xyz(0.0, 100.0, 0.0), // Example starting position
                ..Default::default()
            },
            rigid_body: RigidBody::Dynamic,
            collider: Collider::ball(50.0), // Example radius
            restitution: Restitution::coefficient(0.7),
        },
        // Add any other components specific to the ball here
    ));
}
