use bevy::{prelude::*, render::texture::ImageSettings};
// use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

use crate::{ai::*, level::*, player::*, projectile::ProjectilePlugin, ui::*};

pub fn create_app() {
    App::new()
        // Set antialiasing to use 4 samples
        // .insert_resource(Msaa { samples: 4 })
        // Set WindowDescriptor Resource to change title and size
        .insert_resource(WindowDescriptor {
            title: "WarmVector".to_string(),
            fit_canvas_to_parent: true,
            ..default()
        })
        // background color
        .insert_resource(ClearColor(Color::rgb(0.9, 0.5, 0.5)))
        .insert_resource(ImageSettings::default_nearest())
        // exit the game if press ESCAPE
        .add_system(bevy::window::close_on_esc)
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO,
            ..default()
        })
        .add_plugin(LevelPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(AiPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(ProjectilePlugin)
        // TODO: enable in dev?
        // .add_plugin(WorldInspectorPlugin::new())
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
}
