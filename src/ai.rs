use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    core_ext::{AngleExt, RandRectPoint},
    level::setup_level,
    level::Level,
    player::{EntityName, Person, PLAYER_SIZE},
};

struct AiChangeDirectionTimer(Timer);

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default)]
struct AiData {
    target: Option<Vec2>,
}

pub struct AiPlugin;
impl Plugin for AiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AiChangeDirectionTimer(Timer::from_seconds(3.0, true)))
            .add_startup_system(spawn_enemies.after(setup_level))
            .add_system(move_ai);
    }
}

fn spawn_enemies(mut commands: Commands, level: Res<Level>, asset_server: Res<AssetServer>) {
    let mut rng = thread_rng();

    let spawn_bounds = level.spawn_bounds();

    let amount = 20;

    for i in 1..amount {
        let rand_pos = rng.rand_rect_point(&spawn_bounds);

        commands
            .spawn()
            .insert(Enemy)
            .insert(AiData::default())
            .insert(Velocity::zero())
            .insert(Collider::ball(PLAYER_SIZE.x / 2.))
            .insert(RigidBody::Dynamic)
            .insert(ExternalForce::default())
            .insert(Damping {
                linear_damping: 0.8,
                angular_damping: 0.8,
            })
            .insert(Person)
            .insert(EntityName(format!("Enemy {}", i).to_string()))
            .insert_bundle(SpriteBundle {
                texture: asset_server.load("images/enemy.png"), // 48x48
                sprite: Sprite {
                    // add tint
                    color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
                    ..default()
                },
                transform: Transform {
                    translation: rand_pos.extend(0.0),
                    // scale: Vec2::splat(PIXELS_PER_METER).extend(1.0),
                    ..default()
                },
                ..default()
            });
    }
}

// const ENEMY_SPEED: f32 = 1000.0;

fn move_ai(
    time: Res<Time>,
    level: Res<Level>,
    mut timer: ResMut<AiChangeDirectionTimer>,
    mut ai_query: Query<(
        &mut ExternalForce,
        &mut Transform,
        &mut Velocity,
        &mut AiData,
    )>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let spawn_bounds = level.spawn_bounds();

        let mut rng = thread_rng();

        for (_, mut transform, mut velocity, mut ai_data) in ai_query.iter_mut() {
            ai_data.target = if rng.gen_bool(0.7) {
                let vec = rng.rand_rect_point(&spawn_bounds);

                transform.rotation =
                    Quat::from_rotation_z((vec - transform.translation.truncate()).vec_angle());
                velocity.angvel = 0.0;

                Some(vec)
            } else {
                None
            }
        }
    }

    // move towards target by applying force, stronger the farther away
    for (mut ext_force, transform, velocity, ai_data) in ai_query.iter_mut() {
        if let Some(target) = ai_data.target {
            // apply force to get to target and stop
            let delta = target - transform.translation.truncate();
            let distance = delta.length();
            // let direction = delta.normalize_or_zero();

            if distance > 40.0 {
                ext_force.force = delta * 1000.0;
            } else {
                // if we're close enough, slow to a stop
                // -velocity.linvel * 1000.0
            }

            // if distance < 2.0 {
            //     let mut rng = thread_rng();
            //     let spawn_bounds = level.spawn_bounds();

            //     ai_data.target = if rng.gen_bool(0.7) {
            //         Some(rng.rand_rect_point(&spawn_bounds))
            //     } else {
            //         None
            //     }
            // }
        } else {
            // slow down to a stop
            // ext_force.force = -velocity.linvel * 1000.0;
        }
    }
}
