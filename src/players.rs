use bevy::prelude::*;
use rand::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player).add_system(move_player);
    }
}

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyMoveTimer(Timer::from_seconds(0.2, true)))
            .add_startup_system(add_enemies)
            .add_system(move_enemies);
    }
}

struct EnemyMoveTimer(Timer);

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

const PLAYER_SIZE: Vec2 = Vec2::new(32., 32.);

fn add_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 0.0),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(60.0, 50.0, 0.0),
                scale: Vec3::new(PLAYER_SIZE.x, PLAYER_SIZE.y, 1.0),
                ..default()
            },
            ..default()
        });
}

fn add_enemies(mut commands: Commands) {
    commands
        .spawn()
        .insert(Enemy)
        .insert(Person)
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
        .insert(Enemy)
        .insert(Person)
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
        .insert(Enemy)
        .insert(Person)
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

const ENEMY_SPEED: f32 = 2.0;
const PLAYER_SPEED: f32 = 1.5;

fn move_enemies(
    time: Res<Time>,
    mut timer: ResMut<EnemyMoveTimer>,
    mut query: Query<&mut Transform, With<Enemy>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();
        for mut transform in &mut query {
            transform.translation.x += rng.gen_range(-10.0..10.0);
            transform.translation.y += rng.gen_range(-10.0..10.0);
        }
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let transform = &mut query.single_mut();

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
        transform.translation.x += dir_x as f32 * PLAYER_SPEED;
        transform.translation.y += dir_y as f32 * PLAYER_SPEED;
    }
}
