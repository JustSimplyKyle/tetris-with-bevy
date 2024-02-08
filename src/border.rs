use bevy::{prelude::*, render::render_resource::PrimitiveTopology, sprite::MaterialMesh2dBundle};

use crate::blocks::blocks::{Board, POINT_SIZE};
pub struct DrawBorderPlugin;

impl Plugin for DrawBorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, draw_borders);
    }
}

#[derive(Component, Copy, Clone)]
pub struct Border;

fn draw_borders(
    board: Res<Board>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let border = Border;
    let board = &board.inner;
    for row in [0, board.len() - 1] {
        let mesh = Mesh::new(PrimitiveTopology::LineList).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![[0., 0., 12.], [board[0].len() as f32, 0., 12.]],
        );
        let material = materials.add(ColorMaterial::from(Color::GRAY));
        let transform = Transform::default()
            .with_scale(Vec3::from_array([POINT_SIZE, POINT_SIZE, -POINT_SIZE]))
            .with_translation(Vec3::from_array([
                0. - POINT_SIZE * 4.5,
                -POINT_SIZE * row as f32 + POINT_SIZE * 9.5,
                0.,
            ]));

        let mesh_bundle = MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform,
            material,
            ..default()
        };
        commands.spawn((border, mesh_bundle));
    }
    for col in [0, board[0].len()] {
        let material = materials.add(ColorMaterial::from(Color::GRAY));
        let transform = Transform::default()
            .with_scale(Vec3::from_array([POINT_SIZE, POINT_SIZE, -POINT_SIZE]))
            .with_translation(Vec3::from_array([
                0. - POINT_SIZE * 4.5,
                0. + POINT_SIZE * 9.5,
                0.,
            ]));

        let mesh = Mesh::new(PrimitiveTopology::LineList).with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [col as f32, 0., 12.],
                [col as f32, -((board.len() - 1) as f32), 12.],
            ],
        );
        let mesh_bundle = MaterialMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform,
            material,
            ..default()
        };
        commands.spawn((border, mesh_bundle));
    }
}
