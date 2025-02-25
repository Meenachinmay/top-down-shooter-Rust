use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
    }
}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}