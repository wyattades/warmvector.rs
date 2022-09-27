use bevy::prelude::*;
use rand::prelude::*;

pub struct PlayersPlugin;
impl Plugin for PlayersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerMoveTimer(Timer::from_seconds(0.2, true)))
            .add_startup_system(add_players)
            .add_system(move_players);
    }
}

struct PlayerMoveTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Name(String);

const PLAYER_SIZE: Vec2 = Vec2::new(32., 32.);

fn add_players(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Name("Elaina Proctor".to_string()))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(10.0, 30.0, 0.0),
                scale: Vec3::new(PLAYER_SIZE.x, PLAYER_SIZE.y, 1.0),
                ..default()
            },
            ..default()
        });

    commands
        .spawn()
        .insert(Player)
        .insert(Name("Renzo Hume".to_string()))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(20.0, 100.0, 0.0),
                scale: Vec3::new(PLAYER_SIZE.x, PLAYER_SIZE.y, 1.0),
                ..default()
            },
            ..default()
        });

    commands
        .spawn()
        .insert(Player)
        .insert(Name("Zayna Nieves".to_string()))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(30.0, 150.0, 0.0),
                scale: Vec3::new(PLAYER_SIZE.x, PLAYER_SIZE.y, 1.0),
                ..default()
            },
            ..default()
        });
}

const PLAYER_SPEED: f32 = 1.5;
fn move_players(
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<PlayerMoveTimer>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut dir_x = 0;
    let mut dir_y = 0;

    if keyboard_input.pressed(KeyCode::Left) {
        dir_x -= 1;
    }
    if keyboard_input.pressed(KeyCode::Right) {
        dir_x += 1;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        dir_y += 1;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        dir_y -= 1;
    }

    if dir_x != 0 || dir_y != 0 {
        for mut transform in query.iter_mut() {
            transform.translation.x += dir_x as f32 * PLAYER_SPEED;
            transform.translation.y += dir_y as f32 * PLAYER_SPEED;
        }
    } else {
        if timer.0.tick(time.delta()).just_finished() {
            let mut rng = thread_rng();
            for mut transform in &mut query {
                transform.translation.x += rng.gen_range(-10.0..10.0);
                transform.translation.y += rng.gen_range(-10.0..10.0);
            }
        }
    }
}
