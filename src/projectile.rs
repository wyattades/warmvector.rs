use bevy::{prelude::*, sprite::collide_aabb::collide};

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
    _dynamic_collider: DynamicCollider,
}

impl ProjectileBundle {
    pub fn new(position: Vec2, angle: f32) -> ProjectileBundle {
        let speed = 3.;

        ProjectileBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: position.extend(0.0),
                    scale: Vec3::new(8., 4., 1.),
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::BLACK,
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
            _dynamic_collider: DynamicCollider,
        }
    }
}

fn check_projectile_collisions(
    receiver_query: Query<(Entity, &Transform, Option<&Player>), With<Person>>,
    projectile_query: Query<(Entity, &Transform, &HurtPerson)>,
    mut commands: Commands,
) {
    for (receiver_ent, receiver_t, maybe_player) in &receiver_query {
        for (projectile_ent, projectile_t, hurt_person) in &projectile_query {
            if collide(
                receiver_t.translation,
                receiver_t.scale.truncate(),
                projectile_t.translation,
                projectile_t.scale.truncate(),
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
