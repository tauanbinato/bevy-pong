use crate::{
    asset_loader::SpriteAssets, movement::MovingObjectRigidBodyDynamicBundle, schedule::InGameSet,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_UP_VELOCITY: Vec2 = Vec2::new(0.0, 250.0);
const PLAYER_DOWN_VELOCITY: Vec2 = Vec2::new(0.0, -250.0);

#[derive(Component, Debug)]
pub struct Player {
    pub id: u32,
}
impl Player {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player).add_systems(
            Update,
            player_movement_controls.in_set(InGameSet::UserInput),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    sprite_assets: Res<SpriteAssets>,
    q_window: Query<&Window>,
) {
    let window = q_window.single();
    // Assuming you want to create boundaries around the entire window
    let window_width = window.width();

    commands.spawn((
        MovingObjectRigidBodyDynamicBundle {
            collider: Collider::cuboid(10.0, 60.0),
            restitution: Restitution::default(),
            model: SpriteBundle {
                texture: sprite_assets.player.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (window_width / 2.0) - 30.0,
                    0.0,
                    0.0,
                )),
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            gravity_scale: GravityScale(0.0),
            mass_properties: ColliderMassProperties::Mass(100.0),
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            velocity: Velocity::default(),
            lock_axes: LockedAxes::TRANSLATION_LOCKED_X | LockedAxes::ROTATION_LOCKED,
            damping: Damping::default(),
        },
        Player { id: 1 },
    ));

    commands.spawn((
        MovingObjectRigidBodyDynamicBundle {
            collider: Collider::cuboid(10.0, 60.0),
            restitution: Restitution::default(),
            model: SpriteBundle {
                texture: sprite_assets.player.clone(),
                transform: Transform::from_translation(Vec3::new(
                    (-window_width / 2.0) + 30.0,
                    0.0,
                    0.0,
                )),
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            gravity_scale: GravityScale(0.0),
            mass_properties: ColliderMassProperties::Mass(100.0),
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            velocity: Velocity::default(),
            lock_axes: LockedAxes::TRANSLATION_LOCKED_X | LockedAxes::ROTATION_LOCKED,
            damping: Damping::default(),
        },
        Player { id: 2 },
    ));
}

/* Set the velocities inside of a system. */
fn player_movement_controls(
    mut velocities: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut vel in velocities.iter_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            vel.linvel = PLAYER_UP_VELOCITY;
            //vel.angvel = 0.4;
        } else if keyboard_input.pressed(KeyCode::KeyS) {
            vel.linvel = PLAYER_DOWN_VELOCITY;
            //vel.angvel = 0.4;
        }
    }
}
