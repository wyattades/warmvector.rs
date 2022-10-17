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
            Option<&Player>,
        ),
        Without<StaticCollider>,
    >,
    static_collider_query: Query<(&StaticCollider, &Transform), Without<DynamicCollider>>,
) {
    for (dynamic_collider, mut dynamic_t, mut entity_velocity, maybe_player) in
        dynamic_collider_query.iter_mut()
    {
        for (static_collider, static_t) in &static_collider_query {
            if let Some(collision) = collide(
                dynamic_t.translation,
                dynamic_collider.size,
                static_t.translation,
                static_collider.size,
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
                } else if reflect_y {
                    entity_velocity.y *= multiplier;
                }

                // TODO: doesn't work
                if (reflect_x || reflect_y) && maybe_player.is_none() {
                    let inverse = entity_velocity.x.signum() * entity_velocity.y.signum();
                    let axis = inverse * if reflect_x { Vec2::Y } else { Vec2::X };

                    // get angle of reflection for entity_velocity
                    let velocity_angle = entity_velocity.angle_between(Vec2::X);
                    let offset_angle = entity_velocity.angle_between(axis);

                    dynamic_t.rotation = Quat::from_rotation_z(velocity_angle * 2. - offset_angle);
                }

                // break;
            }
        }
    }
}
