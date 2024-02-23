use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{ball::Ball, level::Wall, player::Player, schedule::InGameSet};

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_goal, handle_player_hit).in_set(InGameSet::CollisionDetection),
        );
    }
}

fn handle_player_hit(
    mut collision_events: EventReader<CollisionEvent>,
    mut ball_query: Query<(&Ball, &mut Velocity)>,
    player_query: Query<&Player>,
    mut ext_impulses: Query<&mut ExternalImpulse, With<Ball>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // Check if either entity is a player, and the other is a ball.
                let (player_entity, ball_entity) = if player_query.get(*entity1).is_ok()
                    && ball_query.get(*entity2).is_ok()
                {
                    (entity1, entity2)
                } else if player_query.get(*entity2).is_ok() && ball_query.get(*entity1).is_ok() {
                    (entity2, entity1)
                } else {
                    continue; // Skip this event if it doesn't involve a ball-player collision.
                };

                if let Ok((_ball, velocity)) = ball_query.get_mut(*ball_entity) {
                    // Optionally log the player hit
                    if let Ok(player) = player_query.get(*player_entity) {
                        info!("Player {} hit the ball!", player.id);
                    }

                    // Apply an impulse that reflects the ball's incoming velocity
                    // This example simply inverts the velocity vector for the impulse
                    if let Ok(mut ext_impulse) = ext_impulses.get_mut(*ball_entity) {
                        ext_impulse.impulse = -velocity.linvel * -1.9; // Reflect and scale the velocity vector for the impulse
                        ext_impulse.torque_impulse = 0.0;
                    }
                }
            }
            CollisionEvent::Stopped(_entity1, _entity2, _flags) => {
                // Handle collision stop event if needed
            }
        }
    }
}

/* A system that displays the events. */
fn handle_goal(
    mut collision_events: EventReader<CollisionEvent>,
    wall_query: Query<&Wall>,
    ball_query: Query<&Ball>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                // Check if either entity is a wall, and the other is a ball.
                let (wall_entity, _ball_entity) =
                    if wall_query.get(*entity1).is_ok() && ball_query.get(*entity2).is_ok() {
                        (entity1, entity2)
                    } else if wall_query.get(*entity2).is_ok() && ball_query.get(*entity1).is_ok() {
                        (entity2, entity1)
                    } else {
                        continue; // Skip this event if it doesn't involve a ball-wall collision.
                    };

                if let Ok(wall) = wall_query.get(*wall_entity) {
                    //info!("Collision detected with wall: {}", wall.name);
                    if wall.name == "Right" {
                        info!("Player 1 has scored a goal!");
                    } else if wall.name == "Left" {
                        info!("Player 2 has scored a goal!");
                    }
                }

                /* if let Ok(ball) = ball_query.get(*ball_entity) {
                    info!("and ball id: {}", ball.id);
                } */
            }
            CollisionEvent::Stopped(_entity1, _entity2, _flags) => {
                // Handle collision stop event if needed
            }
        }
    }
}
