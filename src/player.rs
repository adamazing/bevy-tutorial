use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::{ascii::{AsciiSheet, spawn_ascii_sprite}, TILE_SIZE};

pub struct PlayerPlugin;

#[derive(Component, Inspectable)]
pub struct Player {
    move_speed: f32
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement);
    }
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>
    ){
    let (player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::W) {
        transform.translation.y += player.move_speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        transform.translation.y -= player.move_speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        transform.translation.x -= player.move_speed * TILE_SIZE * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        transform.translation.x += player.move_speed * TILE_SIZE * time.delta_seconds();
    }

}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player_sprite_index = 1;
    let player = spawn_ascii_sprite(&mut commands, &ascii, player_sprite_index, Color::BLUE, Vec3::new(0.0,0.0,900.0));
    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player{ move_speed: 3.0, });


    let background_sprite_index = 0;
    let background = spawn_ascii_sprite(&mut commands, &ascii,background_sprite_index, Color::CYAN, Vec3::new(0.0,0.0,-1.0));
    commands
        .entity(background)
        .insert(Name::new("Background"));

    commands.entity(player).push_children(&[background]);
}
