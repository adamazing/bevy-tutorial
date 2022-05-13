use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::RED;
pub const RATIO: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

mod ascii;
mod debug;
mod player;

use ascii::AsciiPlugin;
use debug::DebugPlugin;
use player::PlayerPlugin;

struct AppSettings {
    height: f32,
    // width: f32,
    resizable: bool,
    title: String,
}

fn app_config() -> AppSettings {
    return AppSettings {
        height: 600.0,
        // width: 800.0,
        resizable: true,
        title: "Foo".to_string(),
    };
}

fn main() {
    let app_settings = app_config();

    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: app_settings.height * RATIO,
            height: app_settings.height,
            title: app_settings.title,
            resizable: app_settings.resizable,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_plugins(DefaultPlugins)
        .add_plugin(AsciiPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;

    camera.orthographic_projection.right = 1.0 * RATIO;
    camera.orthographic_projection.left = -1.0 * RATIO;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

