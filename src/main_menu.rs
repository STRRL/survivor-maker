use crate::{
    color::{PRIMARY_BACKGROUND_COLOR, PRIMARY_COLOR},
    states::GameState,
};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state(GameState::MainMenu)
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu));
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rootNode = commands.spawn_bundle(NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        ..default()
    });
    rootNode.with_children(|parent| {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: PRIMARY_BACKGROUND_COLOR.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "Start",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: PRIMARY_COLOR,
                    },
                ));
            });
    });
    rootNode.with_children(|parent| {
        parent
            .spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                color: PRIMARY_BACKGROUND_COLOR.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "Exit",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: PRIMARY_COLOR,
                    },
                ));
            });
    });
}
