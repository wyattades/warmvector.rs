use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use easer::functions as easing;
use easer::functions::Easing;

use crate::core_ext::AngleExt;
use crate::level::METERS_PER_PIXEL;
use crate::projectile::spawn_projectile;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_player)
            .insert_resource(PlayerShootTimer(Timer::from_seconds(0.1, true)))
            .add_system(apply_inputs)
            .add_system(move_camera);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct EntityName(pub String);

pub const PLAYER_SIZE: Vec2 = Vec2::new(32. * METERS_PER_PIXEL, 32. * METERS_PER_PIXEL);

fn add_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn()
        .insert(Player)
        .insert(Person)
        .insert(Velocity::zero())
        .insert(EntityName("My Player".to_string()))
        .insert(Collider::ball(PLAYER_SIZE.x / 2.))
        .insert(RigidBody::Dynamic)
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("images/player.png"), // 48x48
            sprite: Sprite::default(),
            transform: Transform {
                translation: Vec3::new(60.0, 50.0, 1.0),
                // scale: Vec2::splat(PIXELS_PER_METER).extend(1.0),
                ..default()
            },
            ..default()
        });
}

const PLAYER_SPEED: f32 = 200.;

pub struct PlayerShootTimer(Timer);

pub fn apply_inputs(
    time: Res<Time>,
    mut player_shoot_timer: ResMut<PlayerShootTimer>,
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut player_query: Query<(&mut Velocity, &Collider, &mut Transform), With<Player>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (velocity, collider, transform) = &mut player_query.single_mut();

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

    velocity.linvel = Vec2::new(dir_x as f32, dir_y as f32) * PLAYER_SPEED;

    let window = windows.get_primary().unwrap();
    if let Some(mouse_pos) = window.cursor_position() {
        let window_dim = Vec2::new(window.width(), window.height());

        let delta = mouse_pos - window_dim * 0.5;
        if delta.x == 0. && delta.y == 0. {
            return;
        }

        let angle = delta.vec_angle();

        // rotate sprite towards the mouse pointer
        transform.rotation = Quat::from_rotation_z(angle);

        // left click to shoot
        if buttons.pressed(MouseButton::Left)
            && player_shoot_timer.0.tick(time.delta()).just_finished()
        {
            let world_pos = transform.translation.truncate()
                + delta.normalize() * (collider.as_ball().unwrap().radius() * 2.);

            spawn_projectile(&mut commands, &asset_server, world_pos, angle);
        }
    }
}

#[derive(Default)]
pub struct CameraFlow {
    vel: Vec2,
    // acc: Vec2,
    rotate_offset: Vec2,
}

fn move_camera(
    player_query: Query<(&Transform, &Velocity), With<Player>>,
    windows: Res<Windows>,
    mut camera_flow: Local<CameraFlow>, // Local variable to the system
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let (player_tran, player_vel) = &player_query.single();

    let camera_transform = &mut camera_query.single_mut();
    // let mut camera_vel = &camera_vel.0;

    const MAX_SPEED: f32 = 40.0;
    const ACCEL_RATE: f32 = 1.0;

    if player_vel.linvel.x == 0. && player_vel.linvel.y == 0. {
        let length_sq = camera_flow.vel.length_squared();
        if length_sq >= ACCEL_RATE * 2. {
            let accel = camera_flow.vel.normalize_or_zero() * ACCEL_RATE;
            camera_flow.vel -= accel;
        } else if length_sq != 0. {
            camera_flow.vel = Vec2::ZERO;
        }
    } else {
        let accel = player_vel.linvel.normalize_or_zero() * ACCEL_RATE;
        camera_flow.vel += accel;
        camera_flow.vel = camera_flow.vel.clamp_length_max(MAX_SPEED);
    }

    let camera_speed = camera_flow.vel.length();
    let ease_length = if camera_speed > 0. {
        easing::Quad::ease_in_out(camera_speed, 0., MAX_SPEED * 1., MAX_SPEED * 1.)
    } else {
        0.
    };
    let ease_vel = camera_flow.vel.normalize_or_zero() * ease_length;

    let window = windows.get_primary().unwrap();

    let window_dim = Vec2::new(window.width(), window.height());

    const MAX_RADIUS: f32 = 100.0;

    if let Some(mouse_pos) = window.cursor_position() {
        let mouse_from_center = mouse_pos - window_dim * 0.5;

        let rotate_radius =
            MAX_RADIUS * (mouse_from_center - ease_vel).length() / (window_dim.x * 0.5);

        camera_flow.rotate_offset = mouse_from_center.normalize_or_zero() * rotate_radius;
    }

    camera_transform.translation =
        (ease_vel + camera_flow.rotate_offset + player_tran.translation.truncate())
            .extend(camera_transform.translation.z);
}
