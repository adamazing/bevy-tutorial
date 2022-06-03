use bevy::prelude::*;

pub struct BestiaryPlugin;

impl Plugin for BestiaryPlugin {
    fn build(&self, app: &mut App){
        //
    }
}

#[derive(Component,Inspectable)]
pub struct Enemy {
    active: bool,
    base_health: u32,
}

pub fn spawn_enemy_for_combat(
    commands: &mut Commands,

) {

}
