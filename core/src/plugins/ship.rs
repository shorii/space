use crate::assets::sprite::*;
use crate::atoms::autonomous::Autonomous;
use crate::atoms::life::Life;
use crate::constant::TIME_STEP;
use crate::organisms::bullet::Bullet;
use crate::organisms::ship::{Ship, ShipProps};
use crate::types::MainSystemDescriptorFactory;
use bevy::core::FixedTimestep;
use bevy::ecs::schedule::{IntoSystemDescriptor, SystemDescriptor};
use bevy::prelude::*;

#[derive(Clone)]
pub struct ShipPlugin {
    pub ship_props: ShipProps,
}

impl MainSystemDescriptorFactory for ShipPlugin {
    fn system_descriptor(&self) -> SystemDescriptor {
        let ship_props = self.ship_props.clone();
        let system = move |mut commands: Commands,
                           asset_server: Res<AssetServer>,
                           texture_atlas_handles: Res<TextureAtlasHandles>,
                           texture_atlases: Res<Assets<TextureAtlas>>| {
            Ship::spawn_with_sprite(
                &mut commands,
                asset_server,
                texture_atlas_handles,
                texture_atlases,
                ship_props.clone(),
            );
        };
        system.into_descriptor()
    }
}

impl Plugin for ShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(Ship::movement_system)
                .with_system(Bullet::movement_system)
                .with_system(Life::movement_system::<Ship>)
                .with_system(Autonomous::movement_system),
        )
        .init_resource::<SpriteHandles>()
        .init_resource::<TextureAtlasHandles>()
        .add_state(StartupState::Setup)
        .add_system_set(SystemSet::on_enter(StartupState::Setup).with_system(load_textures))
        .add_system_set(SystemSet::on_update(StartupState::Setup).with_system(check_textures))
        .add_system_set(SystemSet::on_enter(StartupState::Loaded).with_system(setup_texture_atlas))
        .add_system_set(
            SystemSet::on_enter(StartupState::Finished).with_system(self.system_descriptor()),
        );
    }
}
