// src/background.rs
use bevy::prelude::*;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_background);
    }
}

#[derive(Component)]
pub struct Tile {
    pub grid_x: i32,
    pub grid_y: i32,
}

fn generate_background(
    mut commands: Commands,
    window_query: Query<&Window>,
) {
    let window = window_query.single();

    // Define tile size
    let tile_size = 50.0;
    let border_size = 2.0; // Size of the black border between tiles

    // Calculate how many tiles we need based on window size
    let width_tiles = (window.width() / tile_size).ceil() as i32 + 1;
    let height_tiles = (window.height() / tile_size).ceil() as i32 + 1;

    // Calculate the top-left corner for our grid (to center it)
    let offset_x = -window.width() / 2.0 + tile_size / 2.0;
    let offset_y = -window.height() / 2.0 + tile_size / 2.0;

    // First, create a single black background to serve as our grid lines
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            // Make it cover the entire window
            custom_size: Some(Vec2::new(window.width() + tile_size, window.height() + tile_size)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -0.2), // Place it behind everything else
        ..default()
    });

    // Generate the grid of white tiles
    for y in 0..height_tiles {
        for x in 0..width_tiles {
            // Position for this tile
            let position = Vec3::new(
                offset_x + x as f32 * tile_size,
                offset_y + y as f32 * tile_size,
                -0.1, // Z position (just in front of the black background)
            );

            // Spawn white tile entity
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        // Make the white tiles smaller than the grid cells to create black borders
                        custom_size: Some(Vec2::new(tile_size - border_size, tile_size - border_size)),
                        ..default()
                    },
                    transform: Transform::from_translation(position),
                    ..default()
                },
                Tile {
                    grid_x: x,
                    grid_y: y,
                },
            ));
        }
    }

    println!("Generated background with {}x{} tiles", width_tiles, height_tiles);
}