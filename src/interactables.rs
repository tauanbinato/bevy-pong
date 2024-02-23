use crate::{
    asset_loader::SpriteAssets, health::Health, movement::MovingObjectRigidBodyDynamicBundle,
    schedule::InGameSet, static_object::StaticObjectBundle,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_UP_VELOCITY: Vec2 = Vec2::new(0.0, 250.0);
const PLAYER_DOWN_VELOCITY: Vec2 = Vec2::new(0.0, -250.0);

#[derive(Component, Debug)]
pub struct Interactable {
    pub id: u32,
}
impl Interactable {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

pub struct InteractablePlugin;

impl Plugin for InteractablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, spawn_static_interactables);
    }
}

fn spawn_static_interactables(
    mut commands: Commands,
    _sprite_assets: Res<SpriteAssets>,
    q_window: Query<&Window>,
) {
    let window = q_window.single();
    // Assuming you want to create boundaries around the entire window
    let _window_width = window.width();

    commands
        .spawn(StaticObjectBundle {
            collider: Collider::cuboid(100.0, 100.0),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            global_transform: GlobalTransform::default(),
            restitution: Restitution::default(),
            mass_properties: ColliderMassProperties::default(),
            friction: Friction::default(),
        })
        .insert(Interactable::new(1))
        .insert(Health::new(100.0));
}
