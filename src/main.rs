use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use color::SECONDARY_BACKGROUND_COLOR;
mod color;
mod main_menu;
mod states;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(main_menu::MainMenuPlugin)
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
