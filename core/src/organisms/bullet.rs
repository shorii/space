use crate::assets::sprite::{TextureAtlasHandles, WithSprite};
use crate::atoms::autonomous::{Autonomous, AutonomousProps};
use crate::types::{ComponentProps, EntitySpawner};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::any::TypeId;

#[derive(Clone, Default)]
pub struct BulletProps {
    pub fire_point: Vec3,
    pub asset_path: &'static str,
}

impl ComponentProps for BulletProps {}

#[derive(Component)]
pub struct Bullet;

impl Bullet {
    pub fn movement_system(
        mut commands: Commands,
        mut query: Query<(Entity, &Bullet, &mut Transform)>,
    ) {
        // TODO Collision detection
    }
}

impl EntitySpawner for Bullet {
    fn spawn<'w, 's, 'a>(
        commands: &'a mut Commands<'w, 's>,
        props: impl ComponentProps,
    ) -> EntityCommands<'w, 's, 'a> {
        let mut entity_commands = commands.spawn();
        entity_commands.insert(Bullet {});
        let props = AutonomousProps::default();
        entity_commands.insert(Autonomous {
            speed: props.speed,
            movement: props.movement,
        });
        entity_commands
    }
}

impl WithSprite for Bullet {
    fn get_sprite_sheet_bundle(
        asset_server: Res<AssetServer>,
        texture_atlas_handles: Res<TextureAtlasHandles>,
        texture_atlases: Res<Assets<TextureAtlas>>,
        props: impl ComponentProps,
    ) -> Option<SpriteSheetBundle> {
        let bullet_props: BulletProps = props.typed();
        let atlas_handle = texture_atlas_handles.map.get(&TypeId::of::<TextureAtlas>());
        match atlas_handle {
            Some(ah) => {
                let texture_atlas = texture_atlases.get(ah).unwrap();
                let vendor_handle = asset_server.get_handle(bullet_props.asset_path);
                let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
                Some(SpriteSheetBundle {
                    transform: Transform {
                        translation: bullet_props.fire_point,
                        scale: Vec3::splat(0.1),
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
