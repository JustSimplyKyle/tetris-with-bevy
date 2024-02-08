use bevy::prelude::*;

use crate::{blocks::blocks::Board, GameState};
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityMovement,
    BoardDrawer,
    BoardInitUpdate,
    InfoUpdate,
}

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::BoardInitUpdate,
                InGameSet::UserInput,
                InGameSet::EntityMovement,
                InGameSet::BoardDrawer,
                InGameSet::InfoUpdate,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .configure_sets(
            Update,
            (InGameSet::BoardDrawer).run_if(|x: Res<Board>| x.is_changed()),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::BoardDrawer)
                .before(InGameSet::BoardInitUpdate),
        );
    }
}
