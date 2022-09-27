use bevy::prelude::*;
// use bevy_inspector_egui::WorldInspectorPlugin;

use crate::{players::*, ui::*};

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
        // exit the game if press ESCAPE
        .add_system(bevy::window::close_on_esc)
        // TODO: disable in production?
        // .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayersPlugin)
        .add_plugin(UIPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
}
