use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::{
    level::{METERS_PER_PIXEL, PIXELS_PER_METER},
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
        .insert(Velocity {
            linvel: Vec2::from_angle(angle) * speed,
            ..default()
        })
        .insert(Restitution::coefficient(0.3));
}

fn check_projectile_collisions(
    // rapier_context: Res<RapierContext>,
    mut collision_events: EventReader<CollisionEvent>,
    // receiver_query: Query<(Entity, &DynamicCollider, &Transform, Option<&Player>), With<Person>>,
    // projectile_query: Query<(Entity, &DynamicCollider, &Transform, &HurtPerson)>,
    mut commands: Commands,
) {
    for collision_event in collision_events.iter() {
        if let Some((entity_a, entity_b)) = match collision_event {
            CollisionEvent::Started(a, b, _flags) => Some((a, b)),
            CollisionEvent::Stopped(_a, _b, _flags) => None,
        } {
            println!("Received collision event: {:?} <> {:?}", entity_a, entity_b);
        }
    }

    // for (receiver_ent, receiver_c, receiver_t, maybe_player) in &receiver_query {
    //     for (projectile_ent, projectile_c, projectile_t, hurt_person) in &projectile_query {
    //         if collide(
    //             receiver_t.translation,
    //             receiver_c.size,
    //             projectile_t.translation,
    //             projectile_c.size,
    //         )
    //         .is_some()
    //         {
    //             // TODO: damage system
    //             if hurt_person.damage > 0. {
    //                 if maybe_player.is_some() {
    //                     // TODO
    //                     println!("Player hit!");
    //                 } else {
    //                     commands.entity(receiver_ent).despawn();
    //                 }
    //             }

    //             if hurt_person.destroy_self {
    //                 commands.entity(projectile_ent).despawn();
    //             }

    //             break;
    //         }
    //     }
    // }
}
