use bevy::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::ShapePlugin;
use bevy_rapier2d::prelude::*;

use crate::{ai::*, level::*, player::*, projectile::ProjectilePlugin, ui::*};

pub fn create_app() {
    App::new()
        // Set antialiasing to use 4 samples
        // .insert_resource(Msaa { samples: 4 })
        // background color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.5, 0.5)))
        // exit the game if press ESCAPE
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_plugins(
            DefaultPlugins
                // Set WindowDescriptor Resource to change title and size
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "WarmVector".to_string(),
                        fit_canvas_to_parent: true,
                        ..default()
                    },
                    ..default()
                })
                // pixel art:
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                }),
        )
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            // add physics debugger in dev
            enabled: cfg!(debug_assertions),
            ..default()
        })
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(ShapePlugin)
        // TODO: there doesn't seem to be a way to conditionally add a plugin
        // #[cfg(world_inspector)]
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AiPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ProjectilePlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());
}
