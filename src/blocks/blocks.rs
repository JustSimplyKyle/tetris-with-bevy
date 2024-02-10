use std::{convert::Infallible, time::Duration};

use bevy::prelude::*;
use rand_derive2::RandGen;

use crate::{border::DrawBorderPlugin, schedule::InGameSet, GameState};

use super::{
    drawer::{DrawBlockEvent, DrawBoardPlugin},
    gravity::GravityPlugin,
    movement::MovementPlugin,
};

pub const PREVIEW_COUNT: usize = 5;
pub const POINT_SIZE: f32 = 32.;

pub struct TetrisBlockPlugin;

impl Plugin for TetrisBlockPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .init_resource::<CurrentBlockWithPreview<PREVIEW_COUNT>>()
            .init_resource::<Level>()
            .init_resource::<Lines>()
            .init_resource::<Score>()
            .add_event::<LinesIncrementEvent>()
            .add_plugins(MovementPlugin)
            .add_plugins(GravityPlugin)
            .add_plugins(DrawBoardPlugin)
            .add_plugins(DrawBorderPlugin)
            .add_systems(
                Update,
                (block_spawner::<PREVIEW_COUNT>, clear_line).in_set(InGameSet::BoardInitUpdate),
            )
            .add_systems(Update, (level_up).in_set(InGameSet::InfoUpdate))
            .add_systems(Update, (info_gui, board_tui).in_set(InGameSet::BoardDrawer))
            .add_systems(OnEnter(GameState::GameOver), clear_board);
    }
}

fn clear_board(
    mut board: ResMut<Board>,
    mut level: ResMut<Level>,
    mut lines: ResMut<Lines>,
    mut score: ResMut<Score>,
    mut preview: ResMut<CurrentBlockWithPreview<PREVIEW_COUNT>>,
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
    *preview = CurrentBlockWithPreview::default();
    next_state.set(GameState::InGame);
}

#[derive(Resource)]
pub struct Level(pub u8);

impl Default for Level {
    fn default() -> Self {
        Self(9)
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

#[derive(Resource, Debug)]
pub struct Board {
    pub inner: Vec<Vec<BoardBlockState>>,
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
    #[inline]
    pub const fn is_falling(self) -> bool {
        matches!(self, Self::Falling { .. })
    }
    #[inline]
    pub const fn is_placed(self) -> bool {
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
    pub const fn get_duraiton(&self) -> Duration {
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
        let preview = std::array::from_fn(|_| Block::generate_random());
        Self {
            current: Block::generate_random(),
            preview,
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
                if let Some(cell) = board
                    .inner
                    .get_mut(start_row + i)
                    .and_then(|row| row.get_mut(start_col + j))
                {
                    if cell != &BoardBlockState::Empty {
                        next_state.set(GameState::GameOver);
                        break;
                    }
                    *cell = elem;
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

fn board_tui(
    board: Res<Board>,
    lines: Res<Lines>,
    level: Res<Level>,
    preview: Res<CurrentBlockWithPreview<PREVIEW_COUNT>>,
) {
    println!("{}", *board);
    println!("lines: {}", lines.total_lines);
    println!("level: {}", level.0);
    println!("next_piece: {}", preview.preview.first().unwrap());
}

fn info_gui(
    lines: Res<Lines>,
    level: Res<Level>,
    mut event: EventWriter<DrawBlockEvent>,
    preview: Res<CurrentBlockWithPreview<PREVIEW_COUNT>>,
    mut query: Query<&mut Text>,
    mut commands: Commands,
) {
    let value = format!(
        "lines: {}\nlevel: {}\npreview: ",
        lines.total_lines, level.0,
    );
    let transform = Transform::from_translation(Vec3::from_array([POINT_SIZE * 6., 0., 0.]));
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
            transform,
            ..default()
        });
    } else {
        for mut i in query.iter_mut() {
            for p in i.sections.iter_mut() {
                p.value = value.clone();
            }
        }
    }
    let previews = preview.preview;
    for (u, preview) in previews.iter().enumerate().map(|(x, y)| (x * 3, y)) {
        let board = preview.get_occupied();
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if board[row][col].is_falling() {
                    event.send(DrawBlockEvent {
                        row: row + 3 + u,
                        col: col + 16,
                        block_type: *preview,
                    });
                }
            }
        }
    }
}
