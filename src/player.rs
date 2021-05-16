use bevy::{ecs::component::Component, prelude::*};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_player.system())
            .add_system(handle_keyboard_movement.system());
    }
}

#[derive(Bundle)]
struct PlayerBundle<C: Component> {
    player: Player,
    controls: C,
    #[bundle]
    sprite: SpriteBundle,
}

fn setup_player(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(PlayerBundle {
        player: Player { speed: 100.0 },
        controls: KeyboardMovement {
            up: KeyCode::W,
            down: KeyCode::S,
            left: KeyCode::A,
            right: KeyCode::D,
        },
        sprite: SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        },
    });
}

pub struct Player {
    speed: f32,
}

struct KeyboardMovement {
    up: KeyCode,
    down: KeyCode,
    left: KeyCode,
    right: KeyCode,
}

fn handle_keyboard_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &KeyboardMovement, &mut Transform)>,
) {
    if let Ok((player, controls, mut transform)) = query.single_mut() {
        let mut dir = Vec3::ZERO;

        if keyboard_input.pressed(controls.up) {
            dir.y += 1.0;
        }
        if keyboard_input.pressed(controls.down) {
            dir.y -= 1.0;
        }
        if keyboard_input.pressed(controls.left) {
            dir.x -= 1.0;
        }
        if keyboard_input.pressed(controls.right) {
            dir.x += 1.0;
        }

        transform.translation += dir.normalize_or_zero() * time.delta_seconds() * player.speed;
    }
}
