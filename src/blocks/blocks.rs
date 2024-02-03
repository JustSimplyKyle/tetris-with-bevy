use std::time::Duration;

use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::Stopwatch,
};
use rand_derive2::RandGen;

use crate::GameState;

pub struct TetrisBlockPlugin;

const PREVIEW_COUNT: usize = 1;
impl Plugin for TetrisBlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<DasTimer>()
            .init_resource::<CurrentBlockWithPreview<PREVIEW_COUNT>>()
            .init_resource::<SpeedTimer>()
            .init_resource::<Level>()
            .init_resource::<Lines>()
            .init_resource::<Score>()
            .add_event::<LinesIncrementEvent>()
            .add_systems(
                Update,
                (
                    level_up,
                    block_movement_controls,
                    block_spawner::<PREVIEW_COUNT>,
                    draw_block,
                    clear_line,
                    board_tui,
                    board_gui,
                    block_gravity,
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_systems(Update, clear_board.run_if(in_state(GameState::GameOver)));
    }
}

fn clear_board(
    mut board: ResMut<Board>,
    mut level: ResMut<Level>,
    mut lines: ResMut<Lines>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
    mut entity: Query<Entity, With<Block>>,
    mut block_state: Query<&mut BlockState>,
    mut commands: Commands,
) {
    for entity in entity.iter_mut() {
        commands.entity(entity).despawn_recursive();
    }
    for mut state in block_state.iter_mut() {
        *state = BlockState::Placed;
    }
    *level = Level::default();
    *lines = Lines::default();
    *score = Score::default();
    *board = Board::default();
    next_state.set(GameState::InGame);
}

#[derive(Resource)]
pub struct Level(u8);

impl Default for Level {
    fn default() -> Self {
        Self(29)
    }
}

#[derive(Resource, Default)]
pub struct Lines {
    total_lines: usize,
    current_level_lines: usize,
}

#[derive(Event, Default)]
pub struct LinesIncrementEvent(usize);

fn level_up(
    mut lines_event: EventReader<LinesIncrementEvent>,
    mut lines: ResMut<Lines>,
    mut level: ResMut<Level>,
) {
    for i in lines_event.read().map(|x| x.0) {
        if lines.current_level_lines < 10 {
            lines.current_level_lines += i;
        }
        lines.total_lines += i;
        if lines.current_level_lines >= 10 {
            lines.current_level_lines -= 10;
            level.0 += 1;
        }
    }
}

#[derive(Resource, Event, Default)]
pub struct Score(usize);

#[derive(Resource, Default)]
pub struct SpeedTimer {
    watch: Stopwatch,
}

#[derive(Resource, Default)]
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
                            BoardBlockState::Placed { block_type }
                            | BoardBlockState::Falling { block_type } => block_type.to_string(),
                            BoardBlockState::Empty => String::from(" "),
                        }
                    )
                }) + "\n"
            })
            .collect::<String>();
        write!(f, "{k}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BoardBlockState {
    Placed { block_type: Block },
    Falling { block_type: Block },
    Empty,
}

impl BoardBlockState {
    const fn is_falling(self) -> bool {
        matches!(self, Self::Falling { .. })
    }
    const fn is_placed(self) -> bool {
        matches!(self, Self::Placed { .. })
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

#[derive(Bundle)]
pub struct TetrisBlockBundle {
    block: Block,
    state: BlockState,
}

#[derive(Component, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
pub enum BlockState {
    Placed,
    Falling,
}

impl Level {
    const fn get_duraiton(&self) -> Duration {
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

impl Default for BlockState {
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

#[derive(Resource, Clone)]
pub struct CurrentBlockWithPreview<const T: usize> {
    current: Block,
    preview: [Block; T],
}

impl<const T: usize> Default for CurrentBlockWithPreview<T> {
    fn default() -> Self {
        Self {
            current: Block::generate_random(),
            preview: [Block::generate_random(); T],
        }
    }
}

impl<const T: usize> CurrentBlockWithPreview<T> {
    fn get_and_generate_new_random(&mut self) -> Block {
        let original = self.preview[0];
        self.preview.rotate_left(1);
        self.preview
            .last_mut()
            .map(|x| *x = Block::generate_random());
        self.current = Block::generate_random();
        original
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::T => "T",
                Self::J => "J",
                Self::L => "L",
                Self::I => "I",
                Self::O => "O",
                Self::S => "S",
                Self::Z => "Z",
            }
        )
    }
}

fn block_spawner<const T: usize>(
    state: Query<&BlockState>,
    mut board: ResMut<Board>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut current_block_with_preview: ResMut<CurrentBlockWithPreview<T>>,
) {
    if state.iter().all(|&x| x == BlockState::Placed) {
        let block = current_block_with_preview.get_and_generate_new_random();

        let array_to_insert = block.get_occupied();
        let board_mid_point = board.inner.iter().map(Vec::len).max().unwrap() / 2;
        let offset = array_to_insert.iter().map(|x| x.len()).max().unwrap() / 2;
        let start_row = 0; // example starting row
        let start_col = board_mid_point - offset; // example starting column

        // Inserting the array into the vector
        for (i, row) in array_to_insert.iter().enumerate() {
            for (j, &elem) in row.iter().enumerate() {
                if let Some(row) = board.inner.get_mut(start_row + i) {
                    if let Some(cell) = row.get_mut(start_col + j) {
                        if cell != &BoardBlockState::Empty {
                            next_state.set(GameState::GameOver);
                            break;
                        }
                        *cell = elem;
                    }
                }
            }
        }

        /* Create the ground. */
        commands.spawn(TetrisBlockBundle {
            block,
            state: BlockState::Falling,
        });
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
    if board.is_changed() {
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
    }

    for entity in &mesh_handler {
        commands.entity(entity).despawn_recursive();
    }
}

fn clear_line(mut board: ResMut<Board>, mut lines: EventWriter<LinesIncrementEvent>) {
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
    lines.send(LinesIncrementEvent(move_down));
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

fn rotate_matrix(matrix: Vec<Vec<BoardBlockState>>) -> Vec<Vec<BoardBlockState>> {
    let mut moved = true;
    let len = matrix.len();
    for x in 0..len {
        for y in 0..len {
            if matrix[y][len - 1 - x].is_placed() {
                moved = false;
                break;
            }
        }
    }
    let mut new_piece = matrix.clone();
    for x in 0..len {
        for y in 0..len {
            if moved {
                new_piece[y][len - 1 - x] = matrix[x][y];
            }
        }
    }
    new_piece
}
fn block_movement_controls(
    mut query: Query<(&Block, &mut BlockState), With<Block>>,
    mut board: ResMut<Board>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<DasTimer>,
) {
    let board = &mut board.inner;

    if let Some((block, _)) = query
        .iter_mut()
        .find(|(_, state)| **state == BlockState::Falling)
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
                if timer.speed.elapsed() >= Duration::from_millis(50) {
                    timer.speed.reset();
                    translator();
                }
            }
        };

        if keyboard_input.just_pressed(KeyCode::Up) && block != &Block::O {
            rotate_block(board, block);
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
                    .filter(|x| x.is_falling())
                    .any(|_| true);

                for col in (0..cols).rev() {
                    for row in 0..rows {
                        if (col == 0 && board[row][col].is_falling())
                            || (col != 0 && board[row][col - 1].is_falling())
                        {
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
}

fn rotate_block(board: &mut Vec<Vec<BoardBlockState>>, block: &Block) {
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
                    if board_row < rows
                        && board_col < cols
                        && !board[board_row][board_col].is_placed()
                    {
                        board[board_row][board_col] = cell;
                    }
                }
            }
        }
    }
}

fn board_tui(
    board: Res<Board>,
    lines: Res<Lines>,
    level: Res<Level>,
    preview: Res<CurrentBlockWithPreview<PREVIEW_COUNT>>,
) {
    if board.is_changed() {
        println!("{}", *board);
        println!("lines: {}", lines.total_lines);
        println!("level: {}", level.0);
        println!("next_piece: {}", preview.preview.first().unwrap());
    }
}

fn board_gui(
    board: Res<Board>,
    lines: Res<Lines>,
    level: Res<Level>,
    preview: Res<CurrentBlockWithPreview<PREVIEW_COUNT>>,
    mut query: Query<&mut Text>,
    mut commands: Commands,
) {
    if board.is_changed() {
        let value = format!(
            "lines: {}\nlevel: {}\npreview: {}",
            lines.total_lines,
            level.0,
            preview.preview.first().unwrap()
        );
        if query.is_empty() {
            let style = TextStyle {
                font_size: POINT_SIZE,
                ..Default::default()
            };
            commands.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value,
                        style: style,
                    }],
                    ..default()
                },
                text_anchor: bevy::sprite::Anchor::CenterLeft,
                transform: Transform::from_translation(Vec3::from_array([POINT_SIZE * 6., 0., 0.])),
                ..default()
            });
        } else {
            for mut i in query.iter_mut() {
                for p in i.sections.iter_mut() {
                    p.value = value.clone();
                }
            }
        }
    }
}

fn block_gravity(
    mut query: Query<(&Block, &mut BlockState), With<Block>>,
    mut board_b: ResMut<Board>,
    level: Res<Level>,
    time: Res<Time>,
    mut timer: ResMut<SpeedTimer>,
) {
    if let Some((block, mut state)) = query.iter_mut().find(|x| *x.1 == BlockState::Falling) {
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
