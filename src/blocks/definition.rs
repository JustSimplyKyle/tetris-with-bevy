use crate::blocks::blocks::Block;
use bevy::prelude::*;
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
    pub fn get_positions(&self) -> Vec<[f32; 3]> {
        match self {
            Block::T => {
                vec![
                    [0., 0., 0.],
                    [0., 1., 0.],
                    [3., 0., 0.],
                    [0., 1., 0.],
                    [3., 1., 0.],
                    [3., 0., 0.],
                    [1., 0., 0.],
                    [1., -1., 0.],
                    [2., -1., 0.],
                    [1., 0., 0.],
                    [2., 0., 0.],
                    [2., -1., 0.],
                ]
            }
            Block::L => {
                vec![
                    [0., 0., 0.],
                    [0., 3., 0.],
                    [1., 0., 0.],
                    [1., 0., 0.],
                    [0., 3., 0.],
                    [1., 3., 0.],
                    [1., 0., 0.],
                    [1., 1., 0.],
                    [2., 0., 0.],
                    [2., 0., 0.],
                    [2., 1., 0.],
                    [1., 1., 0.],
                ]
            }
            Block::J => {
                vec![
                    [0., 0., 0.],
                    [0., 3., 0.],
                    [1., 0., 0.],
                    [1., 0., 0.],
                    [0., 3., 0.],
                    [1., 3., 0.],
                    [-1., 0., 0.],
                    [-1., 1., 0.],
                    [0., 0., 0.],
                    [-1., 1., 0.],
                    [0., 1., 0.],
                    [0., 0., 0.],
                ]
            }
            Block::I => {
                vec![
                    [0., 0., 0.],
                    [0., 4., 0.],
                    [1., 0., 0.],
                    [1., 4., 0.],
                    [0., 4., 0.],
                    [1., 0., 0.],
                ]
            }
            Block::O => {
                vec![
                    [0., 0., 0.],
                    [2., 0., 0.],
                    [0., 2., 0.],
                    [0., 2., 0.],
                    [2., 0., 0.],
                    [2., 2., 0.],
                ]
            }
            Block::S => {
                vec![
                    [0., 0., 0.],
                    [0., 1., 0.],
                    [2., 0., 0.],
                    [0., 1., 0.],
                    [2., 1., 0.],
                    [2., 0., 0.],
                    [1., 1., 0.],
                    [1., 2., 0.],
                    [3., 1., 0.],
                    [3., 2., 0.],
                    [3., 1., 0.],
                    [1., 2., 0.],
                ]
            }
            Block::Z => {
                vec![
                    [0., 0., 0.],
                    [2., 0., 0.],
                    [0., -1., 0.],
                    [2., 0., 0.],
                    [0., -1., 0.],
                    [2., -1., 0.],
                    [1., -1., 0.],
                    [3., -1., 0.],
                    [1., -2., 0.],
                    [3., -1., 0.],
                    [1., -2., 0.],
                    [3., -2., 0.],
                ]
            }
        }
    }
}
