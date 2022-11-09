use bevy::prelude::*;
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{FillMode, *},
};
use bevy_rapier2d::prelude::*;
use geo::{coord, Rect};

use crate::{core_ext::RectExt, player::PLAYER_SIZE};

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Level {
            bounds: Rect::new(coord! {x: 10., y: 10.}, coord! {x: 1200., y: 800.}),
        })
        .add_startup_system(setup_level);
    }
}

pub struct Level {
    pub bounds: Rect<f32>,
}

impl Level {
    pub fn spawn_bounds(&self) -> Rect<f32> {
        self.bounds
            .expand(-(PLAYER_SIZE.x / 2. + WALL_THICKNESS / 2.))
    }
}

pub const WALL_THICKNESS: f32 = 10.;
const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    #[bundle]
    shape_bundle: ShapeBundle,
    // sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

pub const PIXELS_PER_METER: f32 = 0.8;
pub const METERS_PER_PIXEL: f32 = 1.0 / PIXELS_PER_METER;

impl WallBundle {
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
            shape_bundle: GeometryBuilder::build_as(
                &shapes::Rectangle {
                    extents: size,
                    ..default()
                },
                DrawMode::Fill(FillMode::color(WALL_COLOR)),
                Transform {
                    translation: position.extend(0.0),
                    ..default()
                },
            ),
            collider: Collider::cuboid(size.x * 0.5, size.y * 0.5),
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
