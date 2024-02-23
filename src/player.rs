use crate::{asset_loader::SpriteAssets, movement::MovingObjectRigidBodyDynamicBundle};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(50.0, 0.0, 0.0);
const PLAYER_UP_VELOCITY: Vec2 = Vec2::new(0.0, 250.0);
const PLAYER_DOWN_VELOCITY: Vec2 = Vec2::new(0.0, -250.0);

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_movement_controls);
    }
}

fn spawn_player(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands.spawn((
        MovingObjectRigidBodyDynamicBundle {
            collider: Collider::cuboid(10.0, 60.0),
            restitution: Restitution::coefficient(1.0),
            model: SpriteBundle {
                texture: sprite_assets.player.clone(),
                transform: Transform::from_translation(STARTING_TRANSLATION),
                ..default()
            },
            rigid_body: RigidBody::Dynamic,
            gravity_scale: GravityScale(0.0),
            mass_properties: ColliderMassProperties::default(),
            external_force: ExternalForce::default(),
            external_impulse: ExternalImpulse::default(),
            velocity: Velocity::default(),
            lock_axes: LockedAxes::ROTATION_LOCKED,
        },
        Player,
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
