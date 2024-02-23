/// This module contains components, bundles, and systems related to movement in the game.
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

/// Bundle declaration for general moving objects.
#[derive(Bundle)]
pub struct StaticObjectBundle {
    pub collider: Collider,
    // Include Transform directly in your bundle
    pub transform: Transform,
    // GlobalTransform is necessary for Bevy to calculate global positions
    pub global_transform: GlobalTransform,
    pub restitution: Restitution,
    pub mass_properties: ColliderMassProperties,
    pub friction: Friction,
}
