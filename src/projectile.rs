use bevy::{prelude::*, sprite::collide_aabb::collide};
use rand::prelude::*;

use crate::{
    entity::{DynamicCollider, Velocity},
    player::{Person, Player},
};

pub struct ProjectilePlugin;
impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_projectile_collisions);
    }
}

#[derive(Component)]
struct HurtPerson {
    damage: f32,
    destroy_self: bool,
}

#[derive(Bundle)]
pub struct ProjectileBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    hurt_person: HurtPerson,
    collider: DynamicCollider,
}

impl ProjectileBundle {
    pub fn new(position: Vec2, angle: f32, asset_server: &AssetServer) -> ProjectileBundle {
        let speed = 3.;

        let mut rng = thread_rng();

        let scale = 4.;
        let size = Vec2::new(5., 3.) * scale;

        ProjectileBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load("images/bullet.png"),
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec2::splat(scale).extend(1.0),
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
                    ..default()
                },
                ..default()
            },
            // collider: Collider,
            velocity: Velocity(Vec2::from_angle(angle) * speed),
            hurt_person: HurtPerson {
                damage: 1.,
                destroy_self: true,
            },
            collider: DynamicCollider { size },
        }
    }
}

fn check_projectile_collisions(
    receiver_query: Query<(Entity, &DynamicCollider, &Transform, Option<&Player>), With<Person>>,
    projectile_query: Query<(Entity, &DynamicCollider, &Transform, &HurtPerson)>,
    mut commands: Commands,
) {
    for (receiver_ent, receiver_c, receiver_t, maybe_player) in &receiver_query {
        for (projectile_ent, projectile_c, projectile_t, hurt_person) in &projectile_query {
            if collide(
                receiver_t.translation,
                receiver_c.size,
                projectile_t.translation,
                projectile_c.size,
            )
            .is_some()
            {
                // TODO: damage system
                if hurt_person.damage > 0. {
                    if maybe_player.is_some() {
                        // TODO
                        println!("Player hit!");
                    } else {
                        commands.entity(receiver_ent).despawn();
                    }
                }

                if hurt_person.destroy_self {
                    commands.entity(projectile_ent).despawn();
                }

                break;
            }
        }
    }
}
