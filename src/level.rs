use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ball::Ball, schedule::InGameSet, static_object::StaticObjectBundle};

const WALL_THICKNESS: f32 = 10.0;

#[derive(Component, Debug)]
pub struct Wall {
    pub name: String,
}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_level_boundaries);
    }
}

fn spawn_level_boundaries(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    // Assuming you want to create boundaries around the entire window
    let thickness = WALL_THICKNESS; // Thickness of the walls
    let half_thickness = thickness * 0.5;
    let window_width = window.width();
    let window_height = window.height();
    info!(
        "Window width: {} | Window height: {}",
        window_width, window_height
    );
    // Top wall
    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(window_width * 0.5, half_thickness),
            transform: Transform::from_xyz(0.0, window_height / 2.0 - half_thickness, 0.0),
            global_transform: GlobalTransform::default(),
            restitution: Restitution::default(),
            mass_properties: ColliderMassProperties::Mass(500.0),
        })
        .insert(Wall {
            name: "Top".to_string(),
        });

    // Bottom wall
    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(window_width * 0.5, half_thickness),
            transform: Transform::from_xyz(0.0, -window_height / 2.0 + half_thickness, 0.0),
            global_transform: GlobalTransform::default(),
            restitution: Restitution::default(),
            mass_properties: ColliderMassProperties::Mass(500.0),
        })
        .insert(Wall {
            name: "Bottom".to_string(),
        });

    // Left wall
    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(half_thickness, window_height * 0.5 - thickness), // Subtract thickness to avoid overlap
            transform: Transform::from_xyz(-window_width / 2.0 + half_thickness, 0.0, 0.0),
            global_transform: GlobalTransform::default(),
            restitution: Restitution::default(),
            mass_properties: ColliderMassProperties::Mass(500.0),
        })
        .insert(Wall {
            name: "Left".to_string(),
        })
        .insert(ActiveEvents::COLLISION_EVENTS);

    // Right wall
    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(half_thickness, window_height * 0.5 - thickness), // Subtract thickness to avoid overlap
            transform: Transform::from_xyz(window_width / 2.0 - half_thickness, 0.0, 0.0),
            global_transform: GlobalTransform::default(),
            restitution: Restitution::default(),
            mass_properties: ColliderMassProperties::Mass(500.0),
        })
        .insert(Wall {
            name: "Right".to_string(),
        })
        .insert(ActiveEvents::COLLISION_EVENTS);
}
