use bevy::prelude::*;

use crate::GameState;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityMovement,
    BoardDrawer,
    BoardInitUpdate,
    ScoreLevelUpdate,
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
                InGameSet::ScoreLevelUpdate,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::BoardDrawer)
                .before(InGameSet::BoardInitUpdate),
        );
    }
}
