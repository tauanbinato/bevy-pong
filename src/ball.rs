use crate::{asset_loader::SpriteAssets, movement::MovingObjectRigidBodyDynamicBundle};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const BALL_MAX_SPEED: f32 = 2000.0;

#[derive(Component, Debug)]
pub struct Ball {
    pub id: u32,
}
impl Ball {
    pub fn new(id: u32) -> Self {
        Self { id }
    }
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_ball)
            .add_systems(Update, limit_ball_speed);
    }
}

fn spawn_ball(mut commands: Commands, sprite_assets: Res<SpriteAssets>) {
    commands
        .spawn((
            MovingObjectRigidBodyDynamicBundle {
                model: SpriteBundle {
                    texture: sprite_assets.ball.clone(), // Assuming you have a ball texture in your sprite assets
                    transform: Transform::from_xyz(0.0, 100.0, 0.0), // Example starting position
                    ..Default::default()
                },
                rigid_body: RigidBody::Dynamic,
                collider: Collider::ball(15.0), // Example radius
                restitution: Restitution {
                    coefficient: 2.0,
                    combine_rule: CoefficientCombineRule::Average,
                },
                gravity_scale: GravityScale(0.0),
                mass_properties: ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass: Vec2::new(0.0, 0.0),
                    mass: 5.0,
                    ..Default::default()
                }),
                // Use default if no continuous force is needed
                external_impulse: ExternalImpulse {
                    impulse: Vec2::new(5000.0, 1000.0), // Apply initial impulse here
                    torque_impulse: 0.0,
                },
                friction: Friction::coefficient(0.0),
                external_force: ExternalForce::default(),
                velocity: Velocity::default(),
                lock_axes: LockedAxes::empty(),
                damping: Damping::default(),
            },
            // Add any other components specific to the ball here
        ))
        .insert(Ccd::enabled())
        .insert(Ball::new(1))
        .insert(ActiveEvents::COLLISION_EVENTS);
}

fn limit_ball_speed(mut ball_query: Query<&mut Velocity, With<Ball>>) {
    let mut velocity = ball_query.single_mut();

    // Assuming entity1 is the ball, adjust as necessary
    // Calculate the current speed and direction
    let speed = velocity.linvel.length();
    let max_speed = BALL_MAX_SPEED; // Set your desired max speed here

    // If the current speed exceeds the max speed, scale it down
    if speed > max_speed {
        velocity.linvel = velocity.linvel.normalize() * max_speed;
    }
}
