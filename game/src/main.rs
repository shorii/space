use bevy::prelude::*;
use core::organisms::ship::ShipProps;
use core::plugins::enemy::EnemyPlugin;
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
        .add_plugin(EnemyPlugin {
            spawn_frequency: 1.0 / 1.0,
            asset_paths: vec![
                String::from("textures/Enemy1.png"),
                String::from("textures/Enemy2.png"),
                String::from("textures/Enemy3.png"),
            ],
        })
        .run();
}

// See following URL
// https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
// https://github.com/bevyengine/bevy/blob/main/examples/game/breakout.rs
