use bevy::prelude::*;

use crate::{ascii::{spawn_ascii_sprite, AsciiSheet}, GameState, fadeout::create_fadeout};

#[derive(Component)]
pub struct Enemy;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_system_set(SystemSet::on_update(GameState::Combat).with_system(test_exit_combat))
        .add_system_set(
            SystemSet::on_enter(GameState::Combat)
            .with_system(spawn_enemy)
            .with_system(combat_camera)
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Combat).with_system(despawn_enemy)
        );
    }
}

fn combat_camera(mut camera_query: Query<&mut Transform, With<Camera>>) {
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}

fn spawn_enemy(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let sprite = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        'b' as usize,
        Color::rgb(0.8,0.8,0.8),
        Vec3::new(0.0,0.5,100.0),
    );

    commands.entity(sprite)
        .insert(Enemy)
        .insert(Name::new("Benemy"));
}


fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for entity in enemy_query.iter() {
        print!("Despawning Enemy {:?}",entity);
        commands.entity(entity).despawn_recursive();
    }
}

fn test_exit_combat(
    mut commands: Commands,
    keyboard: ResMut<Input<KeyCode>>,
    ascii: Res<AsciiSheet>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        print!("Transition to Overworld State");
        create_fadeout(&mut commands, Some(GameState::Overworld), &ascii);
    }
}
