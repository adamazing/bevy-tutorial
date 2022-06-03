use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    tilemap::{EncounterSpawner, TileCollider},
    GameState, TILE_SIZE, fadeout::create_fadeout,
};

pub struct PlayerPlugin;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    timer: Timer,
}

#[derive(Component, Inspectable)]
pub struct Player {
    active: bool,
    just_moved: bool,
    move_speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Overworld).with_system(show_player))
            .add_system_set(SystemSet::on_exit(GameState::Overworld).with_system(hide_player))
            .add_system_set(
                SystemSet::on_update(GameState::Overworld)
                    .with_system(player_encounter_checking.after("movement"))
                    .with_system(camera_follow.after("movement"))
                    .with_system(player_movement.label("movement")),
            )
            .add_startup_system(spawn_player);
    }
}

fn show_player(
    mut player_query: Query<(&mut Player, &mut Visibility)>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    // println!("Hiding player...");
    let (mut player, mut player_visibility) = player_query.single_mut();
    player.active = true;
    player_visibility.is_visible = true;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_visibility) = child_visibility_query.get_mut(*child) {
                child_visibility.is_visible = true;
            }
        }
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    // println!("Hiding player...");
    let mut player_visibility = player_query.single_mut();
    player_visibility.is_visible = false;

    if let Ok(children) = children_query.get_single() {
        for child in children.iter() {
            if let Ok(mut child_visibility) = child_visibility_query.get_mut(*child) {
                child_visibility.is_visible = false;
            }
        }
    }
}

/*
 * Encounter checking.
 **/
fn player_encounter_checking(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &mut EncounterTracker, &Transform)>,
    encounter_query: Query<&Transform, (With<EncounterSpawner>, Without<Player>)>,
    ascii: Res<AsciiSheet>,
    time: Res<Time>,
) {
    let (mut player, mut encounter_tracker, player_transform) = player_query.single_mut();
    let player_translation = player_transform.translation;

    if player.just_moved
        && encounter_query
            .iter()
            .any(|&transform| collision_check(player_translation, transform.translation))
    {
        encounter_tracker.timer.tick(time.delta());

        if encounter_tracker.timer.just_finished() {
            player.active = false;
            create_fadeout(&mut commands, Some(GameState::Combat), &ascii);
        }
    }
}

/*
 * Simple camera follow approach, sets camera translation to be that of the player.
 **/
fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

/*
 * Player movement.
 **/
fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    // This is safe as long as there's only one player
    let (mut player, mut transform) = player_query.single_mut();
    player.just_moved = false; // reset to false

    // Don't move if active is set to false
    if !player.active { return; }

    // Calculate the delta in the `y` direction.
    let mut y_delta: f32 = 0.0;
    if keyboard.pressed(KeyCode::W) || keyboard.pressed(KeyCode::Up) {
        y_delta += player.move_speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) || keyboard.pressed(KeyCode::Down) {
        y_delta -= player.move_speed * TILE_SIZE * time.delta_seconds();
    }

    // Calculate the delta in the `x` direction.
    let mut x_delta: f32 = 0.0;
    if keyboard.pressed(KeyCode::A) || keyboard.pressed(KeyCode::Left){
        x_delta -= player.move_speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) || keyboard.pressed(KeyCode::Right){
        x_delta += player.move_speed * TILE_SIZE * time.delta_seconds();
    }

    // Calculate the potential new position, considering only the `x` delta.
    if x_delta != 0.0 {
        player.just_moved = true;
        let target = transform.translation + Vec3::new(x_delta, 0.0, 0.0);
        if !wall_query
            .iter()
            .any(|&transform| collision_check(target, transform.translation))
        {
            transform.translation = target;
        }
    }

    // Calculate the potential new position, considering only the `y` delta.
    if y_delta != 0.0 {
        player.just_moved = true;
        let target = transform.translation + Vec3::new(0.0, y_delta, 0.0);
        if !wall_query
            .iter()
            .any(|&transform| collision_check(target, transform.translation))
        {
            transform.translation = target;
        }
    }
}

/*
 * Check if two tiles are colliding.
 **/
fn collision_check(target_player_pos: Vec3, other_translation: Vec3) -> bool {
    collide(
        target_player_pos,
        Vec2::splat(TILE_SIZE * 0.9),
        other_translation,
        Vec2::splat(TILE_SIZE),
    )
    .is_some()
}

/*
 * Spawns the player.
 **/
fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player_spawn_position = Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 900.0);

    let player_sprite_index = 1;
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        player_sprite_index,
        Color::BLUE,
        player_spawn_position,
    );
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player {
            active: true,
            just_moved: false,
            move_speed: 3.0,
        })
        .insert(EncounterTracker {
            timer: Timer::from_seconds(1.5, true),
        });

    let background_sprite_index = 0;
    let background = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        background_sprite_index,
        Color::CYAN,
        Vec3::new(0.0, 0.0, -1.0), // -1.0 sets background relative to parent (`player`)
    );

    commands.entity(background).insert(Name::new("Background")); // Name the background.
    commands.entity(player).push_children(&[background]); // Push it as a child of player.
}
