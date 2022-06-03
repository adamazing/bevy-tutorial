use bevy::{prelude::*, app::AppExit};

pub struct GameInputPlugin;

impl Plugin for GameInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(handle_game_input);
    }
}

/*
 * Handle more global kinds of input, e.g. for quitting/pausing the game
 *
 **/
fn handle_game_input(
    mut exit: EventWriter<AppExit>,
    keyboard: Res<Input<KeyCode>>
) {
    if keyboard.pressed(KeyCode::Q) || keyboard.pressed(KeyCode::Escape) {
       exit.send(AppExit);
    }
}
