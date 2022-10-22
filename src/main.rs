use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use color::SECONDARY_BACKGROUND_COLOR;
mod color;
mod in_game;
mod main_menu;
mod player;
mod states;

fn main() {
    App::new()
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(main_menu::MainMenuPlugin)
        .add_plugin(in_game::InGamePlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(SECONDARY_BACKGROUND_COLOR),
            ..Default::default()
        },
        ..Default::default()
    });
}
