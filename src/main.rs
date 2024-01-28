mod blocks;
mod board;
mod collision;
use bevy::prelude::*;
use blocks::blocks::TetrisBlockPlugin;
use collision::CollisionPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(TetrisBlockPlugin)
        .add_plugins(CollisionPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const SIZE: f32 = 32.0;
