use crate::blocks::blocks::Block;
use bevy::prelude::*;
use bevy_xpbd_2d::components::Collider;
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

    pub fn get_collider(&self) -> Collider {
        match self {
            Block::T => {
                let top = Collider::convex_hull(vec![
                    Vec2::new(0., 0.),
                    Vec2::new(3., 0.),
                    Vec2::new(0., -1.),
                    Vec2::new(3., -1.),
                ])
                .unwrap();
                let bottom = Collider::convex_hull(vec![
                    Vec2::new(0., 0.),
                    Vec2::new(1., 1.),
                    Vec2::new(1., 0.),
                    Vec2::new(0., 1.),
                ])
                .unwrap();
                Collider::compound(vec![
                    (Vec2::new(0., 1.), 0., top),
                    (Vec2::new(1., -1.), 0., bottom),
                ])
            }
            Block::J => {
                let long = Collider::convex_hull(vec![
                    Vec2::new(0., 3.),
                    Vec2::new(0., 0.),
                    Vec2::new(1., 0.),
                    Vec2::new(1., 3.),
                ])
                .unwrap();
                let p = Collider::convex_hull(vec![
                    Vec2::new(0., 0.),
                    Vec2::new(0., 1.),
                    Vec2::new(1., 1.),
                    Vec2::new(1., 0.),
                ])
                .unwrap();

                Collider::compound(vec![
                    (Vec2::new(0.0, 0.0), 0.0, long),
                    (Vec2::new(-1.0, 0.0), 0.0, p),
                ])
            }
            Block::L => {
                let long = Collider::convex_hull(vec![
                    Vec2::new(0., 3.),
                    Vec2::new(0., 0.),
                    Vec2::new(1., 0.),
                    Vec2::new(1., 3.),
                ])
                .unwrap();
                let p = Collider::convex_hull(vec![
                    Vec2::new(1., 1.),
                    Vec2::new(2., 1.),
                    Vec2::new(2., 0.),
                    Vec2::new(1., 0.),
                ])
                .unwrap();

                Collider::compound(vec![
                    (Vec2::new(0.0, 0.0), 0.0, long),
                    (Vec2::new(0.0, 0.0), 0.0, p),
                ])
            }
            Block::I => Collider::convex_hull(vec![
                Vec2::new(0., 4.),
                Vec2::new(1., 4.),
                Vec2::new(0., 0.),
                Vec2::new(1., 0.),
            ])
            .unwrap(),
            Block::O => Collider::convex_hull(vec![
                Vec2::new(0., 0.),
                Vec2::new(0., 2.),
                Vec2::new(2., 2.),
                Vec2::new(2., 0.),
            ])
            .unwrap(),
            Block::Z => {
                let top = Collider::convex_hull(vec![
                    Vec2::new(0., 0.),
                    Vec2::new(2., 0.),
                    Vec2::new(0., -1.),
                    Vec2::new(2., -1.),
                ])
                .unwrap();
                Collider::compound(vec![
                    (Vec2::new(0., 0.), 0., top.clone()),
                    (Vec2::new(1., -1.), 0., top),
                ])
            }
            Block::S => {
                let bottom = Collider::convex_hull(vec![
                    Vec2::new(0., 0.),
                    Vec2::new(2., 0.),
                    Vec2::new(0., -1.),
                    Vec2::new(2., -1.),
                ])
                .unwrap();
                Collider::compound(vec![
                    (Vec2::new(0., 1.), 0., bottom.clone()),
                    (Vec2::new(1., 2.), 0., bottom),
                ])
            }
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
