use bevy::prelude::*;
use core::plugins::ship::ShipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShipPlugin {
            asset_path: "textures/UFO.png",
        })
        .run();
}

// See following URL
// https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
// https://github.com/bevyengine/bevy/blob/main/examples/game/breakout.rs
