/// This module contains components, bundles, and systems related to movement in the game.
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::schedule::InGameSet;

/// Represents the velocity of an object in 3D space.
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}
impl Velocity {
    /// Creates a new `Velocity` instance with the given value.
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

/// Represents the acceleration of an object in 3D space.
#[derive(Component, Debug)]
pub struct Acceleration {
    pub value: Vec3,
}
impl Acceleration {
    /// Creates a new `Acceleration` instance with the given value.
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

/// Bundle declaration for general moving objects.
#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub velocity: Velocity,
    pub acceleration: Acceleration,
    pub model: SpriteBundle,
    pub collider: Collider,
}

#[derive(Bundle)]
pub struct MovingObjectRigidBodyDynamicBundle {
    // For rendering the entity
    pub model: SpriteBundle,
    // Rapier physics components
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
}

/// Plugin for handling movement-related systems.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_velocity, update_position)
                .chain()
                .in_set(InGameSet::EntityUpdates),
        );
    }
}

/// System that updates the velocity of objects based on their acceleration.
fn update_velocity(mut query: Query<(&Acceleration, &mut Velocity)>, time: Res<Time>) {
    for (acceleration, mut velocity) in query.iter_mut() {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

/// System that updates the position of objects based on their velocity.
fn update_position(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
