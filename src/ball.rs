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
            collider: Collider::ball(15.0), // Example radius
            restitution: Restitution::coefficient(1.0),
            gravity_scale: GravityScale(0.0),
            mass_properties: ColliderMassProperties::Mass(100.0),
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            velocity: Velocity::default(),
            lock_axes: LockedAxes::default(),
        },
        // Add any other components specific to the ball here
    ));
}
