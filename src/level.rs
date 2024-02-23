use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::static_object::StaticObjectBundle;

const WALL_THICKNESS: f32 = 10.0;

#[derive(Component, Debug)]
pub struct Wall;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_level_boundaries);
    }
}

fn spawn_level_boundaries(mut commands: Commands, windows: Query<&Window>) {
    let window = windows.single();
    // Assuming you want to create boundaries around the entire window
    let thickness = WALL_THICKNESS; // Thickness of the walls
    let half_thickness = thickness * 0.5;
    let window_width = window.width();
    let window_height = window.height();

    // Top wall
    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(window_width * 0.5, half_thickness),
            transform: Transform::from_xyz(0.0, window_height / 2.0 - half_thickness, 0.0),
            global_transform: GlobalTransform::default(),
        })
        .insert(Wall);

    // Bottom wall
    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(window_width * 0.5, half_thickness),
            transform: Transform::from_xyz(0.0, -window_height / 2.0 + half_thickness, 0.0),
            global_transform: GlobalTransform::default(),
        })
        .insert(Wall);

    // Left wall
    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(half_thickness, window_height * 0.5 - thickness), // Subtract thickness to avoid overlap
            transform: Transform::from_xyz(-window_width / 2.0 + half_thickness, 0.0, 0.0),
            global_transform: GlobalTransform::default(),
        })
        .insert(Wall);

    // Right wall
    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(half_thickness, window_height * 0.5 - thickness), // Subtract thickness to avoid overlap
            transform: Transform::from_xyz(window_width / 2.0 - half_thickness, 0.0, 0.0),
            global_transform: GlobalTransform::default(),
        })
        .insert(Wall);
}
