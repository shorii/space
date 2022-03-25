use crate::assets::sprite::*;
use crate::atoms::autonomous::{Autonomous, Movement};
use crate::atoms::life::Life;
use crate::constant::TIME_STEP;
use crate::organisms::enemy::{Enemy, EnemyInstanceCount, EnemyProps};
use crate::types::MainSystemDescriptorFactory;
use bevy::core::FixedTimestep;
use bevy::ecs::schedule::{IntoSystemDescriptor, SystemDescriptor};
use bevy::prelude::*;
use rand::seq::SliceRandom;

#[derive(Clone)]
pub struct EnemyPlugin {
    pub spawn_frequency: f64,
    pub asset_paths: Vec<String>,
}

impl MainSystemDescriptorFactory for EnemyPlugin {
    fn system_descriptor(&self) -> SystemDescriptor {
        let asset_paths = self.asset_paths.clone();
        let system = move |mut commands: Commands,
                           asset_server: Res<AssetServer>,
                           texture_atlas_handles: Res<TextureAtlasHandles>,
                           texture_atlases: Res<Assets<TextureAtlas>>| {
            let asset_path = asset_paths.choose(&mut rand::thread_rng()).unwrap();
            let movement: Movement = rand::random();
            Enemy::spawn_with_sprite(
                &mut commands,
                asset_server,
                texture_atlas_handles,
                texture_atlases,
                EnemyProps {
                    spawn_point: Vec3::new(640.0, 0.0, 0.0),
                    asset_path: asset_path.clone(),
                    movement: movement,
                },
            );
        };
        system.into_descriptor()
    }
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EnemyInstanceCount::new(25))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                    .with_system(Enemy::movement_system)
                    .with_system(Life::movement_system::<Enemy>)
                    .with_system(Autonomous::movement_system),
            )
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(self.spawn_frequency))
                    .with_system(self.system_descriptor()),
            );
    }
}
