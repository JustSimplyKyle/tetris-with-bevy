use crate::blocks::blocks::Block;
use bevy::prelude::*;

use super::blocks::BoardBlockState;
impl Block {
    pub const fn get_color(self) -> Color {
        match self {
            Self::T => Color::PURPLE,
            Self::J => Color::BLUE,
            Self::L => Color::ORANGE,
            Self::I => Color::CYAN,
            Self::O => Color::YELLOW,
            Self::S => Color::RED,
            Self::Z => Color::GREEN,
        }
    }

    pub fn get_occupied(self) -> Vec<Vec<BoardBlockState>> {
        use BoardBlockState as E;
        let falling = E::Falling { block_type: self };
        match self {
            Self::T => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![E::Empty, falling, E::Empty],
                vec![falling, falling, falling],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Self::J => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![falling, E::Empty, E::Empty],
                vec![falling, falling, falling],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Self::L => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![E::Empty, E::Empty, falling],
                vec![falling, falling, falling],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Self::O => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![falling, falling, E::Empty],
                vec![falling, falling, E::Empty],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Self::S => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![E::Empty, falling, falling],
                vec![falling, falling, E::Empty],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Self::Z => vec![
                vec![E::Empty, E::Empty, E::Empty],
                vec![falling, falling, E::Empty],
                vec![E::Empty, falling, falling],
                vec![E::Empty, E::Empty, E::Empty],
            ],
            Self::I => vec![
                vec![E::Empty, E::Empty, E::Empty, E::Empty],
                vec![falling, falling, falling, falling],
                vec![E::Empty, E::Empty, E::Empty, E::Empty],
            ],
        }
    }
}
