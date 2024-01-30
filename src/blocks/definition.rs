use crate::blocks::blocks::Block;
use bevy::prelude::*;

use super::blocks::BoardBlockState;
impl Block {
    pub fn get_color(&self) -> Color {
        match self {
            Block::T => Color::PURPLE,
            Block::J => Color::PINK,
            Block::L => Color::ORANGE,
            Block::I => Color::CYAN,
            Block::O => Color::YELLOW,
            Block::S => Color::RED,
            Block::Z => Color::GREEN,
        }
    }

    pub fn rotate_right(&self) -> Vec<Vec<BoardBlockState>> {
        use BoardBlockState as E;
        let falling = E::Falling { block_type: *self };
        match self {
            Block::I => vec![
                vec![E::Empty, E::Empty, falling, E::Empty],
                vec![E::Empty, E::Empty, falling, E::Empty],
                vec![E::Empty, E::Empty, falling, E::Empty],
                vec![E::Empty, E::Empty, falling, E::Empty],
            ],
            _ => todo!(),
        }
    }

    pub fn get_occupied(&self) -> Vec<Vec<BoardBlockState>> {
        use BoardBlockState as E;
        let falling = E::Falling { block_type: *self };
        match self {
            Block::T => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![E::Empty, falling, E::Empty],
                vec![falling, falling, falling],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Block::J => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![falling, E::Empty, E::Empty],
                vec![falling, falling, falling],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Block::L => vec![
                vec![E::Empty, E::Empty, E::Empty, E::Empty],
                vec![E::Empty, E::Empty, E::Empty, falling],
                vec![E::Empty, falling, falling, falling],
                vec![E::Empty, E::Empty, E::Empty, E::Empty],
            ],
            Block::O => vec![
                vec![E::Empty, E::Empty, E::Empty, E::Empty],
                vec![E::Empty, falling, falling, E::Empty],
                vec![E::Empty, falling, falling, E::Empty],
                vec![E::Empty, E::Empty, E::Empty, E::Empty],
            ],
            Block::S => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![E::Empty, falling, falling],
                vec![falling, falling, E::Empty],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Block::Z => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![falling, falling, E::Empty],
                vec![E::Empty, falling, falling],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Block::I => vec![
                vec![E::Empty, E::Empty, E::Empty, E::Empty],
                vec![falling, falling, falling, falling],
                vec![E::Empty, E::Empty, E::Empty, E::Empty],
            ],
        }
    }
}
