mod player;
mod enemy;
mod weapon;
mod score;
mod ui;
mod background;

use bevy::prelude::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use weapon::WeaponPlugin;
use score::ScorePlugin;
use ui::UiPlugin;
use background::BackgroundPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BackgroundPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(WeaponPlugin)
        .add_plugins(ScorePlugin)
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Setup camera
    commands.spawn(Camera2dBundle::default());
}