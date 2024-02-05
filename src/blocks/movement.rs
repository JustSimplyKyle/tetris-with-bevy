use std::{process::exit, time::Duration};

use bevy::{prelude::*, time::Stopwatch};

use crate::schedule::InGameSet;

use super::blocks::{Block, BlockState, Board, BoardBlockState};

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpeedTimer>()
            .init_resource::<DasTimer>()
            .add_systems(
                Update,
                block_movement_controls.in_set(InGameSet::EntityMovement),
            );
    }
}
#[derive(Resource, Default)]
pub struct SpeedTimer {
    pub watch: Stopwatch,
}

#[derive(Resource, Default)]
pub struct DasTimer {
    das: Stopwatch,
    speed: Stopwatch,
}

fn block_movement_controls(
    query: Query<(&Block, &BlockState), With<Block>>,
    mut board: ResMut<Board>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<DasTimer>,
) {
    let board = &mut board.inner;

    let Some((block, _)) = query
        .iter()
        .find(|(_, state)| **state == BlockState::Falling)
    else {
        return;
    };

    let mut das_handler = |keycode, translator: &mut dyn FnMut()| {
        timer.das.tick(time.delta());
        if keyboard_input.just_pressed(keycode) {
            translator();
        }
        if timer.das.elapsed() >= Duration::from_millis(150) {
            timer.speed.tick(time.delta());
            if keyboard_input.just_released(keycode) {
                timer.das.reset();
            }
            if timer.speed.elapsed() >= Duration::from_millis(50) {
                timer.speed.reset();
                translator();
            }
        }
    };

    if keyboard_input.any_just_pressed([KeyCode::Z, KeyCode::Up]) && block != &Block::O {
        rotate_block(board, block, true);
    }
    if keyboard_input.just_pressed(KeyCode::X) && block != &Block::O {
        rotate_block(board, block, false);
    }

    if keyboard_input.pressed(KeyCode::Left) {
        das_handler(KeyCode::Left, &mut || {
            let mut block_allowed_to_move = Vec::with_capacity(4);

            let rows = board.len();
            let cols = board[0].len();

            let at_edge = board
                .iter()
                .map(|x| x[0])
                .find(|x| x.is_falling())
                .is_some();

            for col in 0..cols - 1 {
                for row in 0..rows {
                    if board[row][col + 1].is_falling() {
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
            for col in 1..cols {
                for row in 0..rows {
                    if matches!(board[row][col], BoardBlockState::Falling { .. })
                        && block_allowed_to_move.iter().all(|&x| x)
                        && !at_edge
                    {
                        board[row][col] = BoardBlockState::Empty;
                        board[row][col - 1] = BoardBlockState::Falling { block_type: *block };
                    }
                }
            }
        });
    } else if keyboard_input.pressed(KeyCode::Right) {
        das_handler(KeyCode::Right, &mut || {
            let mut block_allowed_to_move = Vec::with_capacity(4);

            let rows = board.len();
            let cols = board[0].len();

            let at_edge = board
                .iter()
                .filter_map(|x| x.last())
                .find(|x| x.is_falling())
                .is_some();

            for col in (1..cols).rev() {
                for row in 0..rows {
                    // if (col == 0 && board[row][col].is_falling())
                    if (board[row][col - 1].is_falling()) {
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
            for col in (0..cols).rev() {
                for row in 0..rows {
                    if matches!(board[row][col], BoardBlockState::Falling { .. })
                        && block_allowed_to_move.iter().all(|&x| x)
                        && !at_edge
                    {
                        board[row][col] = BoardBlockState::Empty;
                        board[row][col + 1] = BoardBlockState::Falling { block_type: *block };
                    }
                }
            }
        });
    } else {
        timer.das.reset();
    }
}
fn extract_matrix(
    board: &Vec<Vec<BoardBlockState>>,
    top_left: (usize, usize),
    block: Block,
) -> Option<Vec<Vec<BoardBlockState>>> {
    let mut matrix = if block == Block::I {
        vec![vec![BoardBlockState::Empty; 4]; 4]
    } else {
        vec![vec![BoardBlockState::Empty; 3]; 3]
    };
    let mut moved = true;
    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            let board_row = top_left.0 + i;
            let board_col = top_left.1 + j;
            if board_row < board.len() && board_col < board[0].len() {
                *cell = board[board_row][board_col];
            } else {
                moved = false;
                break;
            }
        }
    }
    if moved {
        Some(matrix)
    } else {
        None
    }
}

fn rotate_matrix(matrix: Vec<Vec<BoardBlockState>>, block: Block) -> Vec<Vec<BoardBlockState>> {
    let mut occupied = Vec::new();
    let mut new_piece = matrix.clone();
    let len = matrix.len();
    let rotator = |x| len - 1 - x;
    for x in 0..len {
        for y in 0..len {
            if !matrix[y][rotator(x)].is_placed() && matrix[x][y].is_falling() {
                new_piece[x][y] = BoardBlockState::Empty;
                occupied.push((x, y));
            }
        }
    }
    for x in 0..len {
        for y in 0..len {
            if occupied.contains(&(x, y)) {
                new_piece[y][rotator(x)] = matrix[x][y];
            }
        }
    }
    if new_piece
        .last()
        .unwrap()
        .iter()
        .all(|x| *x == BoardBlockState::Empty)
    {
        new_piece.rotate_right(1);
    }
    let t = if block == Block::I { 3 } else { 1 };
    for _ in 0..t {
        if new_piece
            .iter()
            .filter_map(|x| x.first())
            .all(|x| *x == BoardBlockState::Empty)
        {
            for i in new_piece.iter_mut() {
                i.rotate_left(1);
            }
        } else if new_piece
            .iter()
            .filter_map(|x| x.last())
            .all(|x| *x == BoardBlockState::Empty)
        {
            for i in new_piece.iter_mut() {
                i.rotate_right(1);
            }
        } else {
            break;
        }
    }
    new_piece
}

fn rotate_block(board: &mut Vec<Vec<BoardBlockState>>, block: &Block, clockwise: bool) {
    let rows = board.len();
    let cols = board[0].len();
    let mut vec = Vec::new();
    for row in 0..rows {
        for col in 0..cols {
            if board[row][col].is_falling() {
                vec.push((row, col));
            }
        }
    }
    let (Some(row_min), Some(col_min)) =
        (vec.iter().map(|x| x.0).min(), vec.iter().map(|x| x.1).min())
    else {
        return;
    };

    let top_left = (row_min, col_min);
    let Some(matrix) = extract_matrix(&*board, top_left, *block) else {
        return;
    };
    let rotated = if clockwise {
        rotate_matrix(matrix, *block)
    } else {
        let matrix = rotate_matrix(matrix, *block);
        let matrix = rotate_matrix(matrix, *block);
        rotate_matrix(matrix, *block)
    };
    for (i, row) in rotated.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            let board_row = row_min + i;
            let board_col = col_min + j;
            if board_row < rows && board_col < cols && !board[board_row][board_col].is_placed() {
                board[board_row][board_col] = cell;
            }
        }
    }
}
