use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    level::METERS_PER_PIXEL,
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

pub fn spawn_projectile(
    commands: &mut Commands,
    asset_server: &AssetServer,
    position: Vec2,
    angle: f32,
) {
    let speed = 500.;

    let mut rng = thread_rng();

    commands
        .spawn()
        .insert_bundle(SpriteBundle {
            texture: asset_server.load("images/bullet.png"), // 5x3
            transform: Transform {
                translation: position.extend(0.0),
                // scale: Vec2::splat(PIXELS_PER_METER).extend(1.0),
                rotation: Quat::from_rotation_z(angle),
                ..default()
            },
            sprite: Sprite {
                color: Color::hsl(rng.gen_range(0.0..360.0), 1.0, 0.5),
                ..default()
            },
            ..default()
        })
        .insert(HurtPerson {
            damage: 1.,
            destroy_self: true,
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(
            5. * METERS_PER_PIXEL,
            3. * METERS_PER_PIXEL,
        ))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            linvel: Vec2::from_angle(angle) * speed,
            ..default()
        })
        .insert(Restitution::coefficient(0.3));
}

fn check_projectile_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    hurted_query: Query<(Entity, Option<&Player>), With<Person>>,
    hurter_query: Query<(Entity, &HurtPerson)>,
    mut commands: Commands,
) {
    for collision_event in collision_events.iter() {
        if let Some((entity_a, entity_b)) = match collision_event {
            CollisionEvent::Started(a, b, _flags) => Some((a, b)),
            CollisionEvent::Stopped(_a, _b, _flags) => None,
        } {
            // get the HurtPerson component from entity_a or entity_b
            let (hurter_entity, hurted_entity, hurt_person) =
                if let Ok((_entity, hurt_person)) = hurter_query.get(*entity_a) {
                    (*entity_a, *entity_b, hurt_person)
                } else if let Ok((_entity, hurt_person)) = hurter_query.get(*entity_b) {
                    (*entity_b, *entity_a, hurt_person)
                } else {
                    continue;
                };

            if let Ok((entity, maybe_person)) = hurted_query.get(hurted_entity) {
                println!("Protectile hit person for {:?} damage", hurt_person.damage);
                if !maybe_person.is_some() {
                    commands.entity(entity).despawn();

                    if hurt_person.destroy_self {
                        commands.entity(hurter_entity).despawn();
                    }
                }
            }
        }
    }
}
