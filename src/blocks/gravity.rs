use bevy::prelude::*;

use crate::schedule::InGameSet;

use super::{
    blocks::{Block, BlockState, Board, BoardBlockState, Level},
    movement::SpeedTimer,
};
pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, block_gravity.in_set(InGameSet::EntityMovement));
    }
}

fn block_gravity(
    mut query: Query<(&Block, &mut BlockState), With<Block>>,
    mut board_b: ResMut<Board>,
    level: Res<Level>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut timer: ResMut<SpeedTimer>,
) {
    if let Some((block, mut state)) = query.iter_mut().find(|x| *x.1 == BlockState::Falling) {
        timer.watch.tick(time.delta());

        let pressed = keyboard_input.pressed(KeyCode::Down) && level.0 <= 19;

        if timer.watch.elapsed() >= level.get_duraiton()
            || (pressed && timer.watch.elapsed() >= Level(19).get_duraiton())
        {
            let board = &mut board_b.inner;
            timer.watch.reset();
            let rows = board.len();
            let cols = board[0].len();

            for x in board
                .last_mut()
                .unwrap()
                .iter_mut()
                .filter(|x| x.is_falling())
            {
                *x = BoardBlockState::Placed { block_type: *block };
                *state = BlockState::Placed;
            }

            // first pass, check if it's possible to move down

            let mut block_allowed_to_move = Vec::with_capacity(4);
            for row in (1..rows).rev() {
                for col in 0..cols {
                    // the current dot if moving
                    if board[row - 1][col].is_falling() {
                        // previous dot status
                        match board[row][col] {
                            BoardBlockState::Empty | BoardBlockState::Falling { .. } => {
                                block_allowed_to_move.push(true);
                            }
                            BoardBlockState::Placed { .. } => {
                                block_allowed_to_move.push(false);
                            }
                        }
                    }
                }
            }
            // second pass, move it down without checking
            for row in (1..rows).rev() {
                for col in 0..cols {
                    // Apply gravity from bottom to second row (since the first row can't move down)
                    if matches!(board[row][col], BoardBlockState::Falling { .. }) {
                        if block_allowed_to_move.iter().all(|&x| x) {
                            board[row][col] = BoardBlockState::Empty;
                            board[row + 1][col] = BoardBlockState::Falling { block_type: *block };
                        } else {
                            board[row][col] = BoardBlockState::Placed { block_type: *block };
                            *state = BlockState::Placed;
                        }
                    }
                }
            }
        }
    }
}
