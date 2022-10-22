use crate::{
    color::{ACCENT_COLOR, PRIMARY_BACKGROUND_COLOR, PRIMARY_COLOR},
    states::GameState,
};
use bevy::{app::AppExit, prelude::*};

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state(GameState::MainMenu)
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(clean_menu))
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(setup_menu))
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(hover_color)
                    .with_system(button_start)
                    .with_system(button_exit),
            );
    }
}

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ExitButton;

struct MenuData {
    start_button: Entity,
    exit_button: Entity,
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut root_node = commands.spawn_bundle(NodeBundle {
        style: Style {
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::ColumnReverse,
            ..Default::default()
        },
        ..default()
    });

    let mut start_button_entity_option: Option<Entity> = Option::None;
    root_node.with_children(|parent| {
        let mut start_button = parent.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: PRIMARY_BACKGROUND_COLOR.into(),
            ..Default::default()
        });
        start_button.insert(StartButton {});
        start_button.with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Start",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: PRIMARY_COLOR,
                },
            ));
        });
        start_button_entity_option = Option::Some(start_button.id());
    });

    let mut exit_button_entity_option: Option<Entity> = Option::None;
    root_node.with_children(|parent| {
        let mut exit_button = parent.spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(300.0), Val::Px(65.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: PRIMARY_BACKGROUND_COLOR.into(),
            ..Default::default()
        });
        exit_button.with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Exit",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: PRIMARY_COLOR,
                },
            ));
        });
        exit_button.insert(ExitButton {});
        exit_button_entity_option = Option::Some(exit_button.id());
    });
    if let (Some(start_button_entity), Some(exit_button_entity)) =
        (start_button_entity_option, exit_button_entity_option)
    {
        commands.insert_resource(MenuData {
            start_button: start_button_entity,
            exit_button: exit_button_entity,
        });
    }
}

fn clean_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    commands.entity(menu_data.start_button).despawn_recursive();
    commands.entity(menu_data.exit_button).despawn_recursive();
}

fn hover_color(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match interaction {
            Interaction::Hovered => *color = ACCENT_COLOR.into(),
            Interaction::None => *color = PRIMARY_BACKGROUND_COLOR.into(),
            _ => {}
        }
    }
}

fn button_start(
    mut state: ResMut<State<GameState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<StartButton>)>,
) {
    for interaction in &interaction_query {
        match interaction {
            Interaction::Clicked => state.set(GameState::InGame).unwrap(),
            _ => {}
        }
    }
}

fn button_exit(
    mut exit: EventWriter<AppExit>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ExitButton>)>,
) {
    for interaction in &interaction_query {
        match interaction {
            Interaction::Clicked => {
                exit.send(AppExit);
            }
            _ => {}
        }
    }
}
