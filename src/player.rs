use bevy::prelude::*;
use easer::functions as easing;
use easer::functions::Easing;

use crate::entity::{apply_velocity, Velocity};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player)
            .add_system(apply_inputs)
            .add_system(move_camera.after(apply_velocity));
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct EntityName(pub String);

pub const PLAYER_SIZE: Vec2 = Vec2::new(32., 32.);

fn add_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(Person)
        .insert(Velocity(Vec2::ZERO))
        .insert(EntityName("My Player".to_string()))
        .insert_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::ANTIQUE_WHITE,
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

const PLAYER_SPEED: f32 = 1.5;

pub fn apply_inputs(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    mut player_query: Query<(&mut Velocity, &mut Transform), With<Player>>,
) {
    let (velocity, transform) = &mut player_query.single_mut();

    let mut dir_x = 0;
    let mut dir_y = 0;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        dir_x -= 1;
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        dir_x += 1;
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        dir_y += 1;
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        dir_y -= 1;
    }

    velocity.x = dir_x as f32 * PLAYER_SPEED;
    velocity.y = dir_y as f32 * PLAYER_SPEED;

    // if dir_x != 0 || dir_y != 0 {
    //     transform.translation.x += dir_x as f32 * PLAYER_SPEED;
    //     transform.translation.y += dir_y as f32 * PLAYER_SPEED;
    // }

    // rotate sprite towards the mouse pointer
    let window = windows.get_primary().unwrap();
    if let Some(mouse_pos) = window.cursor_position() {
        let angle = -libm::atan2(
            (mouse_pos.x - window.width() * 0.5).into(),
            (mouse_pos.y - window.height() * 0.5).into(),
        );
        transform.rotation = Quat::from_rotation_z(angle as f32);
    }
}

fn move_camera(
    player_query: Query<(&Transform, &Velocity), With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut Velocity), (With<Camera2d>, Without<Player>)>,
) {
    let (player_tran, player_vel) = &player_query.single();

    let (camera_transform, camera_vel) = &mut camera_query.single_mut();
    // let mut camera_vel = &camera_vel.0;

    let max_speed = 40.0;
    let accel_rate = 1.0;

    if player_vel.x == 0. && player_vel.y == 0. {
        let length_sq = camera_vel.length_squared();
        if length_sq >= accel_rate * 2. {
            let accel = camera_vel.normalize_or_zero() * accel_rate;
            camera_vel.0 -= accel;
        } else if length_sq != 0. {
            camera_vel.0 = Vec2::ZERO;
        }
    } else {
        let accel = player_vel.normalize_or_zero() * accel_rate;
        camera_vel.0 += accel;
        camera_vel.0 = camera_vel.0.clamp_length_max(max_speed);
    }

    let mut new_len = camera_vel.length();
    if new_len > 0. {
        new_len = easing::Quad::ease_in_out(new_len, 0., max_speed * 3., max_speed * 3.);
    }
    let new_vel = camera_vel.normalize_or_zero() * new_len;

    camera_transform.translation.x = player_tran.translation.x - new_vel.x;
    camera_transform.translation.y = player_tran.translation.y - new_vel.y;
}
