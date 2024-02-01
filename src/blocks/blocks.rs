use std::{process::exit, time::Duration};

use bevy::{
    prelude::*,
    reflect::Array,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::Stopwatch,
};
use rand_derive2::RandGen;

pub struct TetrisBlockPlugin;

impl Plugin for TetrisBlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<DasTimer>()
            .init_resource::<Level>()
            .init_resource::<SpeedTimer>()
            .add_systems(Update, block_movement_controls)
            .add_systems(Update, block_spawner)
            .add_systems(Update, draw_block)
            .add_systems(Update, clear_line)
            .add_systems(Update, board_tui)
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

#[derive(Resource, Debug)]
pub struct Board {
    inner: Vec<Vec<BoardBlockState>>,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let k = &self
            .inner
            .iter()
            .map(|i| {
                i.iter().fold(String::new(), |acc, x| {
                    acc + &format!(
                        "[{}]",
                        match x {
                            BoardBlockState::Placed { block_type: x } => x.to_string(),
                            BoardBlockState::Falling { block_type: x } => x.to_string(),
                            BoardBlockState::Empty => String::from(" "),
                        }
                    )
                }) + "\n"
            })
            .collect::<String>();
        write!(f, "{}", k)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoardBlockState {
    Placed { block_type: Block },
    Falling { block_type: Block },
    Empty,
}

impl BoardBlockState {
    fn is_falling(&self) -> bool {
        match self {
            Self::Falling { .. } => true,
            _ => false,
        }
    }
    fn is_placed(&self) -> bool {
        match self {
            Self::Placed { .. } => true,
            _ => false,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            inner: {
                (0..20)
                    .map(|_| (0..10).map(|_| BoardBlockState::Empty).collect())
                    .collect()
            },
        }
    }
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
    facing: Angle,
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum Angle {
    Up,
    Right,
    Down,
    Left,
}

impl Angle {
    fn rotate_right(&self) -> Self {
        match self {
            Angle::Up => Angle::Right,
            Angle::Right => Angle::Down,
            Angle::Down => Angle::Left,
            Angle::Left => Angle::Right,
        }
    }
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
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

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Debug, RandGen, Clone, Copy)]
pub enum Block {
    T,
    J,
    L,
    I,
    O,
    S,
    Z,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Block::T => "T",
                Block::J => "J",
                Block::L => "L",
                Block::I => "I",
                Block::O => "O",
                Block::S => "S",
                Block::Z => "Z",
            }
        )
    }
}

fn block_spawner(state: Query<&State>, mut board: ResMut<Board>, mut commands: Commands) {
    if state.iter().all(|&x| x == State::Placed) {
        let block = Block::generate_random();

        let array_to_insert = block.get_occupied();
        let board_mid_point = board.inner.iter().map(|x| x.len()).max().unwrap() / 2;
        let offset = array_to_insert.iter().map(|x| x.len()).max().unwrap() / 2;
        let start_row = 0; // example starting row
        let start_col = board_mid_point - offset; // example starting column

        // Inserting the array into the vector
        for (i, row) in array_to_insert.iter().enumerate() {
            for (j, &elem) in row.iter().enumerate() {
                if let Some(row) = board.inner.get_mut(start_row + i) {
                    if let Some(cell) = row.get_mut(start_col + j) {
                        *cell = elem;
                    }
                }
            }
        }

        /* Create the ground. */
        commands.spawn((TetrisBlockBundle {
            block,
            state: State::Falling,
            facing: Angle::Up,
        },));
    }
}

const POINT_SIZE: f32 = 32.;

fn draw_block(
    mut commands: Commands,
    board: Res<Board>,
    mesh_handler: Query<Entity, With<Mesh2dHandle>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let block_mesh = meshes.add(Mesh::from(shape::Quad::default()));
    for (u_row, row) in board.inner.iter().enumerate() {
        for (u_col, block) in row.iter().enumerate() {
            match block {
                BoardBlockState::Placed { block_type }
                | BoardBlockState::Falling { block_type } => {
                    let color = block_type.get_color();
                    let material = materials.add(ColorMaterial::from(color));
                    let transform = Transform::default()
                        .with_scale(Vec3::from_array([POINT_SIZE, POINT_SIZE, POINT_SIZE]))
                        .with_translation(Vec3::from_array([
                            POINT_SIZE * u_col as f32 - POINT_SIZE * 4.,
                            -POINT_SIZE * u_row as f32 + POINT_SIZE * 10.,
                            0.,
                        ]));

                    let mesh_bundle = MaterialMesh2dBundle {
                        mesh: block_mesh.clone_weak().into(),
                        transform,
                        material,
                        ..default()
                    };

                    commands.spawn(mesh_bundle);
                }
                BoardBlockState::Empty => {}
            }
        }
    }

    for entity in &mesh_handler {
        commands.entity(entity).despawn_recursive();
    }
}

fn clear_line(mut board: ResMut<Board>) {
    let board = &mut board.inner;
    let p = board
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, x)| x.iter().all(|x| x.is_placed()))
        .map(|x| x.0)
        .collect::<Vec<_>>();

    // clear the lines of impact
    for i in p.iter() {
        board[*i].iter_mut().for_each(|x| {
            *x = BoardBlockState::Empty;
        });
    }

    let move_down = p.iter().count();
    let starting = if move_down != 0 { p[0] } else { 0 };

    let cols = board[0].len();

    for _ in 0..move_down {
        for row in (1..=starting).rev() {
            for col in 0..cols {
                let prev = board[row - 1][col];
                board[row][col] = prev;
            }
        }
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
    let t = if block == Block::I { 3 } else { 1 };
    for _ in 0..t {
        if matrix
            .last()
            .unwrap()
            .iter()
            .all(|x| *x == BoardBlockState::Empty)
        {
            matrix.rotate_right(1);
        }
    }
    let t = if block == Block::I { 1 } else { 1 };
    for _ in 0..t {
        if matrix
            .iter()
            .filter_map(|x| x.last())
            .all(|x| *x == BoardBlockState::Empty)
        {
            for i in matrix.iter_mut() {
                i.rotate_right(1);
            }
        } else if matrix
            .iter()
            .filter_map(|x| x.first())
            .all(|x| *x == BoardBlockState::Empty)
        {
            for i in matrix.iter_mut() {
                i.rotate_left(1);
            }
        }
    }
    if moved {
        Some(matrix)
    } else {
        None
    }
}

fn rotate_matrix<'a>(matrix: Vec<Vec<BoardBlockState>>) -> Vec<Vec<BoardBlockState>> {
    let mut new_piece = matrix.clone();
    let mut moved = true;
    let len = matrix.len();
    for x in 0..len {
        for y in 0..len {
            if new_piece[y][len - 1 - x].is_placed() {
                moved = false;
                break;
            }
        }
    }
    for x in 0..len {
        for y in 0..len {
            if moved {
                new_piece[y][len - 1 - x] = matrix[x][y];
            }
        }
    }
    dbg!(new_piece)
}
fn block_movement_controls(
    mut query: Query<(&Block, &mut State, &mut Angle), With<Block>>,
    mut board: ResMut<Board>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<DasTimer>,
) {
    let board = &mut board.inner;

    if let Some((block, _, mut facing)) = query
        .iter_mut()
        .find(|(_, state, _)| **state == State::Falling)
    {
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
                if timer.speed.elapsed() >= Duration::from_millis(25) {
                    timer.speed.reset();
                    translator();
                }
            }
        };

        if keyboard_input.just_pressed(KeyCode::Up) && block != &Block::O {
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
            let (row_min, col_min) = (vec.iter().map(|x| x.0).min(), vec.iter().map(|x| x.1).min());
            if let (Some(a), Some(b)) = (row_min, col_min) {
                let top_left = (a, b);
                let matrix = extract_matrix(&*board, top_left, *block);
                if let Some(x) = matrix {
                    let rotated = rotate_matrix(x);
                    for (i, row) in rotated.iter().enumerate() {
                        for (j, &cell) in row.iter().enumerate() {
                            let board_row = a + i;
                            let board_col = b + j;
                            if board_row < rows && board_col < cols {
                                if !board[board_row][board_col].is_placed() {
                                    board[board_row][board_col] = cell;
                                }
                            }
                        }
                    }
                }
            }
            // dbg!(vec);
        }

        if keyboard_input.pressed(KeyCode::Left) {
            das_handler(KeyCode::Left, &mut || {
                let mut block_allowed_to_move = Vec::with_capacity(4);

                let rows = board.len();
                let cols = board[0].len();

                let at_edge = board
                    .iter()
                    .map(|x| x[0])
                    .filter(|x| x.is_falling())
                    .any(|_| true);

                for col in 1..cols - 1 {
                    for row in 0..rows {
                        if board[row][col + 1].is_falling() {
                            match board[row][col] {
                                BoardBlockState::Empty => {
                                    block_allowed_to_move.push(true);
                                }
                                BoardBlockState::Placed { .. } => {
                                    block_allowed_to_move.push(false);
                                }
                                // this branch happens if it's the same piece
                                BoardBlockState::Falling { .. } => {
                                    block_allowed_to_move.push(true);
                                }
                            }
                        }
                    }
                }

                // second pass, move it down without checking
                for col in 1..cols {
                    for row in 0..rows {
                        match board[row][col] {
                            BoardBlockState::Falling { .. } => {
                                if block_allowed_to_move.iter().all(|&x| x == true) {
                                    if !at_edge {
                                        board[row][col] = BoardBlockState::Empty;
                                        board[row][col - 1] =
                                            BoardBlockState::Falling { block_type: *block };
                                    }
                                }
                            }
                            _ => {}
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
                    .filter(|x| x.is_falling())
                    .any(|_| true);

                for col in (0..cols).rev() {
                    for row in 0..rows {
                        if (col == 0 && board[row][col].is_falling())
                            || (col != 0 && board[row][col - 1].is_falling())
                        {
                            match board[row][col] {
                                BoardBlockState::Empty => {
                                    block_allowed_to_move.push(true);
                                }
                                BoardBlockState::Placed { .. } => {
                                    block_allowed_to_move.push(false);
                                }
                                // this branch happens if it's the same piece
                                BoardBlockState::Falling { .. } => {
                                    block_allowed_to_move.push(true);
                                }
                            }
                        }
                    }
                }

                // second pass, move it down without checking
                for col in (0..cols).rev() {
                    for row in 0..rows {
                        match board[row][col] {
                            BoardBlockState::Falling { .. } => {
                                if block_allowed_to_move.iter().all(|&x| x == true) {
                                    if !at_edge {
                                        board[row][col] = BoardBlockState::Empty;
                                        board[row][col + 1] =
                                            BoardBlockState::Falling { block_type: *block };
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            });
        } else {
            timer.das.reset();
        }
    }
}

fn board_tui(board: Res<Board>) {
    println!("{}", *board);
}

fn block_gravity(
    mut query: Query<(&Block, &mut State), With<Block>>,
    mut board_b: ResMut<Board>,
    level: Res<Level>,
    time: Res<Time>,
    mut timer: ResMut<SpeedTimer>,
) {
    if let Some((block, mut state)) = query.iter_mut().find(|x| *x.1 == State::Falling) {
        timer.watch.tick(time.delta());

        if timer.watch.elapsed() >= level.get_duraiton() {
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
                *state = State::Placed;
            }

            // first pass, check if it's possible to move down

            let mut block_allowed_to_move = Vec::with_capacity(4);
            for row in (1..rows).rev() {
                for col in 0..cols {
                    // the current dot if moving
                    if board[row - 1][col].is_falling() {
                        // previous dot status
                        match board[row][col] {
                            BoardBlockState::Empty => {
                                block_allowed_to_move.push(true);
                            }
                            BoardBlockState::Placed { .. } => {
                                block_allowed_to_move.push(false);
                            }
                            // this branch happens if it's the same piece
                            BoardBlockState::Falling { .. } => {
                                block_allowed_to_move.push(true);
                            }
                        }
                    }
                }
            }
            // second pass, move it down without checking
            for row in (1..rows).rev() {
                for col in 0..cols {
                    // Apply gravity from bottom to second row (since the first row can't move down)
                    match board[row][col] {
                        BoardBlockState::Falling { .. } => {
                            if block_allowed_to_move.iter().all(|&x| x == true) {
                                board[row][col] = BoardBlockState::Empty;
                                board[row + 1][col] =
                                    BoardBlockState::Falling { block_type: *block };
                            } else {
                                board[row][col] = BoardBlockState::Placed { block_type: *block };
                                *state = State::Placed;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
