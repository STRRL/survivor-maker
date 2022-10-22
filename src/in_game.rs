use bevy::{
    prelude::{
        default, App, AssetServer, Assets, Commands, Component, Deref, DerefMut, Handle, Input,
        KeyCode, ParallelSystemDescriptorCoercion, Plugin, Query, Res, ResMut, SystemSet,
        Transform, Vec2, Vec3, With,
    },
    sprite::{SpriteSheetBundle, TextureAtlas, TextureAtlasSprite},
    time::{Time, Timer},
};

use crate::states::GameState;

#[derive(Component)]
pub struct Player;

pub struct InGamePlugin;

#[derive(Component)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Component)]
pub enum AnimationState {
    Idle,
    Run,
}

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::InGame)
                    .with_system(animate_sprite)
                    .with_system(move_sprite.before(animate_sprite)),
            );
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("textures/rpg/chars/gabe/gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
        .insert(Player)
        .insert(Direction::Right)
        .insert(AnimationState::Idle);
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
        &AnimationState,
        &Direction,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle, animation_state, direction) in &mut query {
        match animation_state {
            AnimationState::Idle => {
                sprite.index = 0;
            }
            AnimationState::Run => {
                timer.tick(time.delta());
                if timer.just_finished() {
                    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                    sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
                }
            }
        }
        match direction {
            Direction::Left => {
                sprite.flip_x = true;
            }
            Direction::Right => {
                sprite.flip_x = false;
            }
        }
    }
}

fn move_sprite(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Transform, &mut AnimationState, &mut Direction), With<Player>>,
) {
    for (mut item, mut animation_state, mut direction) in &mut sprite_position {
        *animation_state = AnimationState::Idle;
        if keyboard_input.pressed(KeyCode::Left) {
            item.translation.x -= 200.0 * time.delta_seconds();
            *animation_state = AnimationState::Run;
            *direction = Direction::Left;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            item.translation.x += 200.0 * time.delta_seconds();
            *animation_state = AnimationState::Run;
            *direction = Direction::Right;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            item.translation.y += 200.0 * time.delta_seconds();
            *animation_state = AnimationState::Run;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            item.translation.y -= 200.0 * time.delta_seconds();
            *animation_state = AnimationState::Run;
        }
    }
}
