use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{border::Border, schedule::InGameSet};

use super::blocks::{Block, Board, BoardBlockState, POINT_SIZE};
pub struct DrawBoardPlugin;

impl Plugin for DrawBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DrawBlockEvent>()
            .add_systems(
                Update,
                (draw_single_block, draw_block).in_set(InGameSet::BoardDrawer),
            )
            .add_systems(Update, clear_blocks.after(InGameSet::BoardDrawer));
    }
}

#[derive(Event)]
pub struct DrawBlockEvent {
    pub row: usize,
    pub col: usize,
    pub block_type: Block,
}

fn clear_blocks(
    mut commands: Commands,
    block_mesh_handler: Query<Entity, (Without<Border>, With<Mesh2dHandle>)>,
) {
    for entity in &block_mesh_handler {
        commands.entity(entity).despawn_recursive();
    }
}

fn draw_block(mut event: EventWriter<DrawBlockEvent>, board: Res<Board>) {
    for (u_row, row) in board.inner.iter().enumerate() {
        for (u_col, block) in row.iter().enumerate() {
            match block {
                BoardBlockState::Placed { block_type }
                | BoardBlockState::Falling { block_type } => event.send(DrawBlockEvent {
                    row: u_row,
                    col: u_col,
                    block_type: *block_type,
                }),
                BoardBlockState::Empty => {}
            }
        }
    }
}

fn draw_single_block(
    mut event: EventReader<DrawBlockEvent>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
) {
    for DrawBlockEvent {
        row,
        col,
        block_type,
    } in event.read()
    {
        let block_mesh = meshes.add(Mesh::from(shape::Quad::default()));
        let color = block_type.get_color();
        let material = materials.add(ColorMaterial::from(color));
        let transform = Transform::default()
            .with_scale(Vec3::from_array([POINT_SIZE, POINT_SIZE, POINT_SIZE]))
            .with_translation(Vec3::from_array([
                POINT_SIZE * *col as f32 - POINT_SIZE * 4.,
                -POINT_SIZE * *row as f32 + POINT_SIZE * 10.,
                0.,
            ]));

        let mesh_bundle = MaterialMesh2dBundle {
            mesh: block_mesh.into(),
            transform,
            material,
            ..default()
        };

        commands.spawn(mesh_bundle);
    }
}
