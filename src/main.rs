pub mod ascii;
pub mod debug;
pub mod player;

use ascii::load_ascii_spritesheet;
use bevy::{prelude::*, render::camera::ScalingMode};
use debug::DebugPlugin;
use player::PlayerPlugin;

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.1;

fn main() {
    let height = 900.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: height * ASPECT_RATIO,
            height,
            ..Default::default()
        })
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_spritesheet)
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugins(DefaultPlugins)
        .run();
}

fn spawn_camera(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * ASPECT_RATIO;
    camera.orthographic_projection.left = -1.0 * ASPECT_RATIO;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}
