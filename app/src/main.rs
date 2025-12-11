use bevy::prelude::*;

mod plugins;
use crate::plugins::startup::StartupPlugin;
use crate::plugins::update::UpdatePlugin;


fn main() {
    // Add logging for wgpu
    // console_error_panic_hook::set_once();
    // std::env::set_var("RUST_LOG", "wgpu_core=trace,wgpu_hal=trace");
    // console_log::init_with_level(log::Level::Debug).unwrap();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Web WASM Runtime".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((StartupPlugin, UpdatePlugin))
        .run();
}