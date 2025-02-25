use bevy::prelude::*;
use crate::score::Score;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_score_text);
    }
}

#[derive(Component)]
struct ScoreText;

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Load a font
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    // If you don't have this font, you can bundle a font with your game
    // or use another font that's available on your system

    // Create UI node for score display
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        // Spawn both the TextBundle and ScoreText component together
        parent.spawn((
            TextBundle::from_sections([
                TextSection::new(
                    "Score: ",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::RED,
                    },
                ),
                TextSection::new(
                    "0",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::GREEN,
                    },
                ),
            ]),
            ScoreText,
        ));
    });
}

fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        let mut text = query.single_mut();
        text.sections[1].value = score.value.to_string();
    }
}