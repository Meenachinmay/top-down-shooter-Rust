use bevy::prelude::*;
use rand::prelude::*;
use rand::{rng, thread_rng};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, enemy_movement);
    }
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub direction: Vec2,
    pub change_direction_timer: Timer,
}

fn spawn_enemies(mut commands: Commands) {
    let mut rng = rng();

    // Spawn 5 enemies at random positions
    for _ in 0..5 {
        let x = rng.random_range(-400.0..400.0);
        let y = rng.random_range(-300.0..300.0);

        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0), // Blue color
                    custom_size: Some(Vec2::new(30.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..default()
            },
            Enemy {
                speed: 100.0,
                direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.random_range(-1.0..1.0)).normalize(),
                change_direction_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
            },
        ));
    }
}

fn enemy_movement(
    time: Res<Time>,
    mut query: Query<(&mut Enemy, &mut Transform)>,
    window_q: Query<&Window>,
) {
    let window = window_q.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;
    let mut rng = thread_rng();

    for (mut enemy, mut transform) in query.iter_mut() {
        // Update timer and change direction if needed
        enemy.change_direction_timer.tick(time.delta());
        if enemy.change_direction_timer.finished() {
            enemy.direction = Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
        }

        // Move enemy
        let translation = &mut transform.translation;
        translation.x += enemy.direction.x * enemy.speed * time.delta().as_secs_f32();
        translation.y += enemy.direction.y * enemy.speed * time.delta().as_secs_f32();

        // Bounce off screen edges
        if translation.x < -half_width || translation.x > half_width {
            enemy.direction.x = -enemy.direction.x;
        }
        if translation.y < -half_height || translation.y > half_height {
            enemy.direction.y = -enemy.direction.y;
        }

        // Clamp position to screen
        translation.x = translation.x.clamp(-half_width + 15.0, half_width - 15.0);
        translation.y = translation.y.clamp(-half_height + 15.0, half_height - 15.0);
    }
}