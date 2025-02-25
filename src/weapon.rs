use bevy::prelude::*;
use rand::prelude::*;
use rand::rng;
use crate::player::Player;
use crate::enemy::Enemy;
use crate::score::Score;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (fire_weapon, bullet_movement, bullet_collision));
    }
}

#[derive(Component)]
pub struct Weapon {
    pub cooldown: Timer,
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub damage: u32,
    pub direction: Vec2,
}

fn fire_weapon(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Weapon), With<Player>>,
) {
    let (player_transform, mut weapon) = query.single_mut();
    weapon.cooldown.tick(time.delta());

    if mouse_button.pressed(MouseButton::Left) && weapon.cooldown.finished() {
        weapon.cooldown.reset();

        // Get the forward direction using the player's rotation
        let forward_dir = (player_transform.rotation * Vec3::X).truncate();

        // Calculate spawn position (in front of the player)
        let spawn_point = player_transform.translation + Vec3::new(forward_dir.x, forward_dir.y, 0.0) * 20.0;

        // Spawn the bullet
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 0.0), // Yellow
                    custom_size: Some(Vec2::new(10.0, 5.0)),
                    ..default()
                },
                transform: Transform {
                    translation: spawn_point,
                    rotation: player_transform.rotation,
                    ..default()
                },
                ..default()
            },
            Bullet {
                speed: 400.0,
                damage: 1,
                direction: forward_dir, // Using the calculated forward direction
            },
        ));
    }
}

fn bullet_movement(
    time: Res<Time>,
    mut commands: Commands,
    mut bullet_query: Query<(Entity, &mut Transform, &Bullet)>,
    window_q: Query<&Window>,
) {
    let window = window_q.single();
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (entity, mut transform, bullet) in bullet_query.iter_mut() {
        // Move the bullet
        transform.translation.x += bullet.direction.x * bullet.speed * time.delta().as_secs_f32();
        transform.translation.y += bullet.direction.y * bullet.speed * time.delta().as_secs_f32();

        // Despawn bullets that go off screen
        if transform.translation.x < -half_width || transform.translation.x > half_width ||
            transform.translation.y < -half_height || transform.translation.y > half_height {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform, &Bullet)>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut score: ResMut<Score>,
) {
    for (bullet_entity, bullet_transform, _) in bullet_query.iter() {
        let bullet_pos = bullet_transform.translation.truncate();

        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            let enemy_pos = enemy_transform.translation.truncate();
            let distance = bullet_pos.distance(enemy_pos);

            // Simple collision detection - if bullet is within 20 units of enemy center
            if distance < 20.0 {
                // Despawn the bullet
                commands.entity(bullet_entity).despawn();

                // Despawn the enemy
                commands.entity(enemy_entity).despawn();

                // Increase the score
                score.value += 10;

                // Spawn a new enemy at a random position
                let mut rng = rng();
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
                        direction: Vec2::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize(),
                        change_direction_timer: Timer::from_seconds(3.0, TimerMode::Repeating),
                    },
                ));

                break;
            }
        }
    }
}