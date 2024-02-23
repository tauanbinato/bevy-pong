use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod asset_loader;
mod ball;
mod camera;
mod collisions;
mod health;
mod interactables;
mod level;
mod movement;
mod player;
mod schedule;
mod state;
mod static_object;

use asset_loader::AssetLoaderPlugin;
use ball::BallPlugin;
use camera::CameraPlugin;
use collisions::CollisionsPlugin;
use interactables::InteractablePlugin;
use level::LevelPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugins((
            MovementPlugin,
            AssetLoaderPlugin,
            PlayerPlugin,
            StatePlugin,
            SchedulePlugin,
            CameraPlugin,
            BallPlugin,
            LevelPlugin,
            InteractablePlugin,
            CollisionsPlugin,
        ))
        .run();
}
