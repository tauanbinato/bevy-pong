/// This module contains components, bundles, and systems related to movement in the game.
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::schedule::InGameSet;

#[derive(Bundle)]
pub struct MovingObjectRigidBodyDynamicBundle {
    // For rendering the entity
    pub model: SpriteBundle,
    // Rapier physics components
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub restitution: Restitution,
    pub gravity_scale: GravityScale,
    pub mass_properties: ColliderMassProperties,
    pub external_force: ExternalForce,
    pub external_impulse: ExternalImpulse,
    pub velocity: Velocity,
    pub lock_axes: LockedAxes,
    pub damping: Damping,
    pub friction: Friction,
}

/// Plugin for handling movement-related systems.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, _app: &mut App) {}
}
