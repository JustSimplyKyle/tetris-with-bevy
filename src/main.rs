mod blocks;
use bevy::prelude::*;
use blocks::blocks::TetrisBlockPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(TetrisBlockPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
