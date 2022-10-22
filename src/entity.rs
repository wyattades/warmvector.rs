use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::player::{apply_inputs, Person};

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_collisions.after(apply_inputs))
            .add_system(apply_velocity.after(check_collisions));
    }
}

#[derive(Component)]
pub struct StaticCollider {
    pub size: Vec2,
}

#[derive(Component)]
pub struct DynamicCollider {
    pub size: Vec2,
}

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

pub fn apply_velocity(mut velocity_query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in velocity_query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn check_collisions(
    mut dynamic_collider_query: Query<
        (
            &DynamicCollider,
            &mut Transform,
            &mut Velocity,
            Option<&Person>,
        ),
        Without<StaticCollider>,
    >,
    static_collider_query: Query<(&StaticCollider, &Transform), Without<DynamicCollider>>,
) {
    for (dynamic_collider, mut dynamic_t, mut entity_velocity, maybe_person) in
        dynamic_collider_query.iter_mut()
    {
        for (static_collider, static_t) in &static_collider_query {
            if let Some(collision) = collide(
                dynamic_t.translation,
                dynamic_collider.size,
                static_t.translation,
                static_collider.size,
            ) {
                let mut reflect_x = false;
                let mut reflect_y = false;

                // only reflect if the velocity is going in the opposite
                // direction of the collision
                match collision {
                    Collision::Left => reflect_x = entity_velocity.x > 0.0,
                    Collision::Right => reflect_x = entity_velocity.x < 0.0,
                    Collision::Top => reflect_y = entity_velocity.y < 0.0,
                    Collision::Bottom => reflect_y = entity_velocity.y > 0.0,
                    Collision::Inside => {}
                }

                let multiplier = if maybe_person.is_some() { 0. } else { -1. };
                if reflect_x {
                    entity_velocity.x *= multiplier;
                } else if reflect_y {
                    entity_velocity.y *= multiplier;
                }

                if (reflect_x || reflect_y) && maybe_person.is_none() {
                    let angle = -entity_velocity.0.angle_between(Vec2::X);

                    dynamic_t.rotation = Quat::from_rotation_z(angle);
                }

                // break;
            }
        }
    }

    // dynamic on dynamic
    let mut combos = dynamic_collider_query.iter_combinations_mut();
    while let Some(
        [(collider_a, mut transform_a, mut velocity_a, mp_a), (collider_b, mut transform_b, mut velocity_b, mp_b)],
    ) = combos.fetch_next()
    {
        if let Some(_collision) = collide(
            transform_a.translation,
            collider_a.size,
            transform_b.translation,
            collider_b.size,
        ) {
            if mp_a.is_none() && mp_b.is_none() {
                println!("collide non-players");
                let old_a = velocity_a.clone();
                velocity_a.x = velocity_b.x;
                velocity_a.y = velocity_b.y;
                velocity_b.x = old_a.x;
                velocity_b.y = old_a.y;

                transform_a.rotation = Quat::from_rotation_z(-velocity_a.0.angle_between(Vec2::X));
                transform_b.rotation = Quat::from_rotation_z(-velocity_b.0.angle_between(Vec2::X));
            } else {
                // match collision {
                //     Collision::Inside => {
                //         println!("collide inside");
                //         continue;
                //     }
                //     _ => {}
                // }
                // velocity_a.x *= 0.;
                // velocity_a.y *= 0.;
                // velocity_b.x *= 0.;
                // velocity_b.y *= 0.;
            }
        }
    }
}
