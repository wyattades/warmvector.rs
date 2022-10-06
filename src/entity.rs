use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::player::{apply_inputs, Player};

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(check_collisions.after(apply_inputs))
            .add_system(apply_velocity.after(check_collisions));
    }
}

#[derive(Component)]
pub struct Collider;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

pub fn apply_velocity(mut velocity_query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in velocity_query.iter_mut() {
        transform.translation.x += velocity.x;
        transform.translation.y += velocity.y;
    }
}

fn check_collisions(
    mut entity_query: Query<(&Transform, &mut Velocity, Option<&Player>), Without<Collider>>,
    collider_query: Query<&Transform, &mut Collider>,
) {
    for (entity_transform, mut entity_velocity, maybe_player) in entity_query.iter_mut() {
        for collider in &collider_query {
            if let Some(collision) = collide(
                entity_transform.translation,
                entity_transform.scale.truncate(),
                collider.translation,
                collider.scale.truncate(),
            ) {
                // reflect the ball when it collides
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

                let multiplier = if maybe_player.is_some() { 0. } else { -1. };
                if reflect_x {
                    entity_velocity.x *= multiplier;
                }
                if reflect_y {
                    entity_velocity.y *= multiplier;
                }

                // break;
            }
        }
    }
}
