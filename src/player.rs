use bevy::prelude::*;
use crate::weapon::Weapon;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement, player_aim));
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

fn spawn_player(mut commands: Commands) {
    // Spawn the player entity
    let player_entity = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0), // Red color
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
        Player { speed: 200.0 },
        Weapon { cooldown: Timer::new(std::time::Duration::from_secs_f32(0.5), TimerMode::Repeating) },
    )).id();

    // Add a gun (white line) as a child of the player
    commands.entity(player_entity).with_children(|parent| {
        parent.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0), // White color
                custom_size: Some(Vec2::new(20.0, 3.0)), // A thin rectangular line
                ..default()
            },
            transform: Transform {
                // Position the gun to extend from the center of the player
                // The gun will stick out of the player square by 10 pixels
                translation: Vec3::new(15.0, 0.0, 0.1),
                ..default()
            },
            ..default()
        });
    });
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();
    let mut direction = Vec3::ZERO;

    // WASD movement
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    // Normalize direction to prevent diagonal movement from being faster
    if direction != Vec3::ZERO {
        direction = direction.normalize();
    }

    transform.translation += direction * player.speed * time.delta().as_secs_f32();
}

fn player_aim(
    window_q: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    // Get required components
    let window = window_q.single();
    let (camera, camera_transform) = camera_q.single();
    let mut player_transform = player_q.single_mut();

    // Get cursor position
    if let Some(cursor_position) = window.cursor_position() {
        // Convert screen coordinates to world coordinates
        if let Some(cursor_world) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            // Get player position
            let player_pos = player_transform.translation.truncate();

            // Calculate vector from player to cursor
            let to_cursor = cursor_world - player_pos;

            // Only update rotation if cursor is far enough from player
            if to_cursor.length_squared() > 4.0 {
                // Calculate angle between player and cursor
                let angle = f32::atan2(to_cursor.y, to_cursor.x);

                // Apply rotation directly - no interpolation
                player_transform.rotation = Quat::from_rotation_z(angle);

                // You can uncomment these debug lines to see what's happening
                // println!("Cursor: ({:.1}, {:.1}), Direction: ({:.2}, {:.2})",
                //     cursor_world.x, cursor_world.y, to_cursor.x, to_cursor.y);
                // println!("Angle: {:.1}° ({:.2} radians)",
                //     angle * 180.0 / std::f32::consts::PI, angle);
            }
        }
    }
}