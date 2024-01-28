use std::{f32::consts::PI, time::Duration};

use bevy::{
    prelude::*, render::render_resource::PrimitiveTopology, sprite::MaterialMesh2dBundle,
    time::Stopwatch,
};
use rand_derive2::RandGen;

use crate::{collision::Collider, SIZE};

pub struct TetrisBlockPlugin;

impl Plugin for TetrisBlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DasTimer>()
            .init_resource::<Level>()
            .init_resource::<SpeedTimer>()
            .add_systems(Update, block_spawner)
            .add_systems(Update, block_movement_controls)
            .add_systems(Update, handle_block_collisions)
            .add_systems(Update, block_gravity);
    }
}

#[derive(Resource)]
pub struct Level(u8);

impl Default for Level {
    fn default() -> Self {
        Self(9)
    }
}

#[derive(Resource)]
pub struct SpeedTimer {
    watch: Stopwatch,
}

#[derive(Resource)]
pub struct DasTimer {
    das: Stopwatch,
    speed: Stopwatch,
}

impl Default for DasTimer {
    fn default() -> Self {
        Self {
            das: Stopwatch::default(),
            speed: Stopwatch::default(),
        }
    }
}

impl Default for SpeedTimer {
    fn default() -> Self {
        Self {
            watch: Stopwatch::default(),
        }
    }
}

#[derive(Bundle)]
pub struct TetrisBlockBundle {
    block: Block,
    state: State,
    collider: Collider,
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum State {
    Placed,
    Falling,
}

impl Level {
    fn get_duraiton(&self) -> Duration {
        let frames: u64 = match self.0 {
            0 => 48,
            1 => 43,
            2 => 38,
            3 => 33,
            4 => 28,
            5 => 23,
            6 => 18,
            7 => 13,
            8 => 8,
            9 => 6,
            10..=12 => 5,
            13..=15 => 4,
            16..=18 => 3,
            19..=28 => 2,
            _ => 1,
        };
        Duration::from_millis(frames * 1000 / 60)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::Placed
    }
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Debug, RandGen)]
pub enum Block {
    T,
    J,
    L,
    I,
    O,
    S,
    Z,
}

fn block_spawner(
    state: Query<&State>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if state.iter().all(|&x| x == State::Placed) {
        let block = Block::generate_random();
        let positions = block.get_positions();
        let color = block.get_color();

        let mesh = Mesh::new(PrimitiveTopology::TriangleList)
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        let material = materials.add(ColorMaterial::from(color));
        let transform = Transform::default()
            .with_scale(Vec3::splat(SIZE))
            .with_translation(Vec3::from_array([0., SIZE * 12., 0.]));

        commands.spawn((
            TetrisBlockBundle {
                block,
                state: State::Falling,
                collider: Collider::new(),
            },
            MaterialMesh2dBundle {
                mesh: meshes.add(mesh).into(),
                transform,
                material,
                ..default()
            },
        ));
    }
}

fn block_movement_controls(
    mut query: Query<(&mut Transform, &mut State), With<Block>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<DasTimer>,
) {
    if let Some((mut piece, _)) = query
        .iter_mut()
        .find(|(_, state)| **state == State::Falling)
    {
        if keyboard_input.any_just_pressed([KeyCode::Z, KeyCode::Up]) {
            let location = dbg!(piece.translation);
            piece.rotate_around(
                location,
                Quat::from_axis_angle(Vec3::from_array([0., 0., 1.]), -PI / 2.),
            );
        }
        if keyboard_input.just_pressed(KeyCode::X) {
            piece.rotate_z(PI / 2.);
        }

        let mut das_handler = |keycode, movement| {
            timer.das.tick(time.delta());
            if keyboard_input.just_pressed(keycode) {
                piece.translation.x += movement;
            }
            if timer.das.elapsed() >= Duration::from_millis(250) {
                timer.speed.tick(time.delta());
                if keyboard_input.just_released(keycode) {
                    timer.das.reset();
                }
                if timer.speed.elapsed() >= Duration::from_millis(25) {
                    timer.speed.reset();
                    piece.translation.x += movement;
                }
            }
        };

        if keyboard_input.pressed(KeyCode::Left) {
            das_handler(KeyCode::Left, -SIZE);
        } else if keyboard_input.pressed(KeyCode::Right) {
            das_handler(KeyCode::Right, SIZE);
        } else {
            timer.das.reset();
        }
    }
}

fn handle_block_collisions(
    query: Query<&Collider, With<Block>>,
    mut query2: Query<&mut State, With<Block>>,
) {
    for i in query.iter().flat_map(|x| x.colliding_entities.iter()) {
        if let Ok(mut p) = query2.get_mut(*i) {
            *p = State::Placed;
        }
    }
}

fn block_gravity(
    mut query: Query<(&mut Transform, &mut State), With<Block>>,
    level: Res<Level>,
    time: Res<Time>,
    mut timer: ResMut<SpeedTimer>,
) {
    for (mut transform, mut state) in query.iter_mut() {
        if *state == State::Falling {
            timer.watch.tick(time.delta());
            if timer.watch.elapsed() >= level.get_duraiton() {
                timer.watch.reset();
                transform.translation.y -= SIZE;
            }
        }
        if transform.translation.y <= SIZE * -6. {
            *state = State::Placed;
        }
    }
}
