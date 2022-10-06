use bevy::prelude::*;
use geo::{coord, Rect};

use crate::entity::Collider;

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level {
            bounds: Rect::new(coord! {x: 10., y: 10.}, coord! {x: 800., y: 600.}),
        })
        .add_startup_system(setup_level);
    }
}

pub struct Level {
    pub bounds: Rect<f32>,
}

const WALL_THICKNESS: f32 = 10.0;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    #[bundle]
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(level: &Level, location: WallLocation) -> WallBundle {
        let geo::Coordinate { x: ax, y: ay } = level.bounds.min();
        let geo::Coordinate { x: bx, y: by } = level.bounds.max();
        let mx = (ax + bx) * 0.5;
        let my = (ay + by) * 0.5;

        let position = match location {
            WallLocation::Left => Vec2::new(ax, my),
            WallLocation::Right => Vec2::new(bx, my),
            WallLocation::Bottom => Vec2::new(mx, ay),
            WallLocation::Top => Vec2::new(mx, by),
        };

        let size = match location {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, level.bounds.height() + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(level.bounds.width() + WALL_THICKNESS, WALL_THICKNESS)
            }
        };

        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    // make sure to add the `z` value
                    translation: position.extend(0.0),
                    scale: size.extend(1.0),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub fn setup_level(mut commands: Commands, level: Res<Level>) {
    // Walls
    commands.spawn_bundle(WallBundle::new(&level, WallLocation::Left));
    commands.spawn_bundle(WallBundle::new(&level, WallLocation::Right));
    commands.spawn_bundle(WallBundle::new(&level, WallLocation::Bottom));
    commands.spawn_bundle(WallBundle::new(&level, WallLocation::Top));
}
