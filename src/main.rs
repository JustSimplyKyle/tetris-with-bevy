mod blocks;
mod board;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use blocks::blocks::TetrisBlockPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .add_systems(Startup, setup)
        .add_plugins(TetrisBlockPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const SIZE: f32 = 32.0;
