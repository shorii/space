use bevy::prelude::*;
use core::organisms::ship::ShipProps;
use core::plugins::ship::ShipPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShipPlugin {
            ship_props: ShipProps {
                speed: 500.0,
                life: 5,
                ship_asset_path: "textures/Ship.png",
                bullet_asset_path: "textures/Bullet.png",
            },
        })
        .run();
}

// See following URL
// https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
// https://github.com/bevyengine/bevy/blob/main/examples/game/breakout.rs
