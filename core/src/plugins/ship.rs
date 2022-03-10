use crate::atoms::bullet::Bullet;
use crate::constant::TIME_STEP;
use crate::organisms::ship::Ship;
use crate::resources::sprite::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

pub struct ShipPlugin {
    pub asset_path: &'static str,
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(Ship::movement_system)
                .with_system(Bullet::movement_system),
        )
        .init_resource::<SpriteHandles>()
        .add_state(StartupState::Setup)
        .add_system_set(SystemSet::on_enter(StartupState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(StartupState::Setup).with_system(check_textures))
        .add_system_set(
            SystemSet::on_enter(StartupState::Finished)
                .with_system(Ship::setup_sprite(self.asset_path)),
        );
    }
}
