use std::f32::consts::PI;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    entity::{DynamicCollider, Velocity},
    level::setup_level,
    level::Level,
    player::{EntityName, Person, PLAYER_SIZE},
};

struct AiChangeDirectionTimer(Timer);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
struct AiData {
    // TODO
}

pub struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AiChangeDirectionTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(spawn_enemies.after(setup_level))
            .add_system(move_ai);
    }
}

fn spawn_enemies(mut commands: Commands, level: Res<Level>) {
    let mut rng = thread_rng();

    let spawn_bounds = level.bounds.clone();
    let padding = 10.0;
    spawn_bounds.min().x += PLAYER_SIZE.x / 2. + padding;
    spawn_bounds.max().x -= PLAYER_SIZE.x / 2. + padding;
    spawn_bounds.min().y += PLAYER_SIZE.y / 2. + padding;
    spawn_bounds.max().y -= PLAYER_SIZE.y / 2. + padding;
    // let padding = 10.0;

    for i in 1..10 {
        commands
            .spawn()
            .insert(Enemy)
            .insert(AiData {})
            .insert(Velocity(Vec2::ZERO))
            .insert(DynamicCollider)
            .insert(Person)
            .insert(EntityName(format!("Enemy {}", i).to_string()))
            .insert_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen_range(spawn_bounds.min().x..spawn_bounds.max().x),
                        rng.gen_range(spawn_bounds.min().y..spawn_bounds.max().y),
                        0.0,
                    ),
                    scale: Vec3::new(PLAYER_SIZE.x, PLAYER_SIZE.y, 1.0),
                    ..default()
                },
                ..default()
            });
    }
}

const ENEMY_SPEED: f32 = 1.0;

fn move_ai(
    time: Res<Time>,
    mut timer: ResMut<AiChangeDirectionTimer>,
    mut ai_query: Query<&mut Velocity, With<AiData>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();
        for mut velocity in &mut ai_query {
            velocity.clone_from(&if rng.gen_bool(0.7) {
                Vec2::from_angle(rng.gen_range(0.0..(2.0 * PI))).clamp_length_min(ENEMY_SPEED)
            } else {
                Vec2::ZERO
            });
        }
    }
}
