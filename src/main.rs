pub mod ascii;
pub mod camera;
pub mod collisions;
pub mod debug;
pub mod enemy;
pub mod magic_missiles;
pub mod physics3d;
pub mod player;
pub mod shield;

use ascii::load_ascii_spritesheet;
use bevy::prelude::*;
use collisions::update_collisions;
use debug::DebugPlugin;
use enemy::EnemyPlugin;
use magic_missiles::MagicMisslesPlugin;
use physics3d::Physics3dPlugin;
use player::PlayerPlugin;
use shield::ShieldPlugin;

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
        .add_plugins(DefaultPlugins)
        // .add_plugin(GameCameraPlugin)
        .add_plugin(Physics3dPlugin)
        .add_plugin(MagicMisslesPlugin)
        .add_plugin(ShieldPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(EnemyPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii_spritesheet)
        .add_system(update_collisions)
        .run();
}
