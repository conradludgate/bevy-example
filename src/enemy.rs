use bevy::prelude::*;

use crate::player::Player;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(enemy_spawner.system())
            .add_system(handle_enemy_spawner.system())
            .add_system(handle_enemy.system());
    }
}

struct EnemySpawner {
    timer: Timer,
    amount: usize,
}

fn enemy_spawner(mut commands: Commands) {
    commands.spawn().insert(EnemySpawner {
        timer: Timer::from_seconds(5.0, true),
        amount: 1,
    });
}

fn handle_enemy_spawner(
    time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&mut EnemySpawner>,
) {
    if let Ok(mut spawner) = query.single_mut() {
        if spawner.timer.tick(time.delta()).just_finished() {
            // Spawn a bunch of enemies
            for _ in 0..spawner.amount {
                commands.spawn_bundle(EnemyBundle {
                    enemy: Enemy { speed: rand::random::<f32>() * 50.0 + 50.0 },
                    sprite: SpriteBundle {
                        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
                        transform: Transform::from_xyz(
                            rand::random::<f32>() * 300.0 - 150.0,
                            rand::random::<f32>() * 300.0 - 150.0,
                            0.0,
                        ),
                        ..Default::default()
                    },
                });
            }

            // Increase next round duration and amount
            spawner.amount *= 2;
            let dur = spawner.timer.duration() * 3 / 2;
            spawner.timer.set_duration(dur);
        }
    }
}

struct Enemy {
    speed: f32,
}

#[derive(Bundle)]
struct EnemyBundle {
    enemy: Enemy,
    #[bundle]
    sprite: SpriteBundle,
}

// move towards the player
fn handle_enemy(
    time: Res<Time>,
    // query for all enemies and for the single player
    mut query_set: QuerySet<(
        Query<(&Enemy, &mut Transform)>,
        Query<(&Player, &Transform)>,
    )>,
) {
    if let Ok(player_translation) = query_set
        .q1()
        .single()
        .map(|(_, Transform { translation, .. })| translation.clone())
    {
        for (enemy, mut transform) in query_set.q0_mut().iter_mut() {
            let dir = player_translation - transform.translation;
            transform.translation += dir.normalize_or_zero() * time.delta_seconds() * enemy.speed;
        }
    }
}
