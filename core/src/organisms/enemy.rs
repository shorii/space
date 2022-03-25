use crate::assets::sprite::{TextureAtlasHandles, WithSprite};
use crate::atoms::autonomous::{Autonomous, Direction, Movement};
use crate::types::{ComponentProps, EntitySpawner};
use anyhow::{bail, Result};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::any::TypeId;

pub struct EnemyInstanceCount {
    limit: usize,
    count: usize,
}

impl EnemyInstanceCount {
    pub fn new(limit: usize) -> EnemyInstanceCount {
        EnemyInstanceCount {
            limit: limit,
            count: 0,
        }
    }
    pub fn decrement(&mut self) -> Result<()> {
        if self.count == 0 {
            bail!("Over instance count limitation.")
        }
        self.count -= 1;
        Ok(())
    }

    pub fn increment(&mut self) -> Result<()> {
        if self.limit <= self.count {
            bail!("Over instance count limitation.")
        }
        self.count += 1;
        Ok(())
    }
}

#[derive(Clone)]
pub struct EnemyProps {
    pub spawn_point: Vec3,
    pub asset_path: String,
    pub movement: Movement,
}

impl ComponentProps for EnemyProps {}

#[derive(Component)]
pub struct Enemy;

impl Enemy {
    pub fn movement_system(
        mut commands: Commands,
        mut query: Query<(Entity, &Enemy, &mut Transform)>,
    ) {
        // TODO Collision detection
    }
}

impl EntitySpawner for Enemy {
    fn spawn<'w, 's, 'a>(
        commands: &'a mut Commands<'w, 's>,
        props: impl ComponentProps,
    ) -> EntityCommands<'w, 's, 'a> {
        let enemy_props = props.typed::<EnemyProps>();
        let mut entity_commands = commands.spawn();
        entity_commands.insert(Enemy {});
        entity_commands.insert(Autonomous {
            speed: 100.0,
            movement: enemy_props.movement,
            direction: Direction::Left,
        });
        entity_commands
    }
}

impl WithSprite for Enemy {
    fn get_sprite_sheet_bundle(
        asset_server: Res<AssetServer>,
        texture_atlas_handles: Res<TextureAtlasHandles>,
        texture_atlases: Res<Assets<TextureAtlas>>,
        props: impl ComponentProps,
    ) -> Option<SpriteSheetBundle> {
        let enemy_props: EnemyProps = props.typed();
        let atlas_handle = texture_atlas_handles.map.get(&TypeId::of::<TextureAtlas>());
        match atlas_handle {
            Some(ah) => {
                let texture_atlas = texture_atlases.get(ah).unwrap();
                let vendor_handle = asset_server.get_handle(enemy_props.asset_path);
                let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
                Some(SpriteSheetBundle {
                    transform: Transform {
                        translation: enemy_props.spawn_point,
                        scale: Vec3::splat(0.2),
                        ..Default::default()
                    },
                    sprite: TextureAtlasSprite::new(vendor_index),
                    texture_atlas: ah.clone(),
                    ..Default::default()
                })
            }
            None => None,
        }
    }
}
