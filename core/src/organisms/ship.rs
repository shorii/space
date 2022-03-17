use crate::assets::sprite::{TextureAtlasHandles, WithSprite, WithSpriteExt};
use crate::atoms::life::Life;
use crate::constant::TIME_STEP;
use crate::organisms::bullet::{Bullet, BulletProps};
use crate::types::{ComponentProps, EntitySpawner};
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use std::any::TypeId;

#[derive(Clone)]
pub struct ShipProps {
    pub speed: f32,
    pub life: u32,
}

impl ComponentProps for ShipProps {}

impl Default for ShipProps {
    fn default() -> Self {
        Self {
            speed: 500.0,
            life: 5,
        }
    }
}

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
}

impl Ship {
    pub fn movement_system(
        mut commands: Commands,
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&Ship, &mut Transform)>,
        asset_server: Res<AssetServer>,
        texture_atlas_handles: Res<TextureAtlasHandles>,
        texture_atlases: Res<Assets<TextureAtlas>>,
    ) {
        match query.get_single_mut() {
            Ok((ship, mut transform)) => {
                let mut direction = 0.0;
                if keyboard_input.pressed(KeyCode::Right) {
                    direction += 1.0;
                    transform.translation.x += direction * ship.speed * TIME_STEP;
                    transform.translation.x = transform.translation.x.min(640.0).max(-640.0);
                    return;
                }
                if keyboard_input.pressed(KeyCode::Left) {
                    direction -= 1.0;
                    transform.translation.x += direction * ship.speed * TIME_STEP;
                    transform.translation.x = transform.translation.x.min(640.0).max(-640.0);
                    return;
                }
                if keyboard_input.pressed(KeyCode::Down) {
                    direction -= 1.0;
                    transform.translation.y += direction * ship.speed * TIME_STEP;
                    transform.translation.y = transform.translation.y.min(380.0).max(-380.0);
                    return;
                }
                if keyboard_input.pressed(KeyCode::Up) {
                    direction += 1.0;
                    transform.translation.y += direction * ship.speed * TIME_STEP;
                    transform.translation.y = transform.translation.y.min(380.0).max(-380.0);
                    return;
                }
                if keyboard_input.pressed(KeyCode::Space) {
                    let mut fire_point = transform.translation;
                    fire_point.x += 30.0;
                    Bullet::spawn_with_sprite(
                        &mut commands,
                        "textures/food_lemon_01.png",
                        asset_server,
                        texture_atlas_handles,
                        texture_atlases,
                        Some(BulletProps {
                            fire_point: fire_point,
                        }),
                    );
                }
            }
            Err(_) => {
                return;
            }
        }
    }
}

impl EntitySpawner for Ship {
    fn spawn<'w, 's, 'a>(
        commands: &'a mut Commands<'w, 's>,
        props: Option<impl ComponentProps>,
    ) -> EntityCommands<'w, 's, 'a> {
        let ship_props: ShipProps = match props {
            Some(p) => p.typed(),
            None => ShipProps::default(),
        };
        let mut entity_commands = commands.spawn();
        entity_commands.insert(Ship {
            speed: ship_props.speed,
        });
        entity_commands.insert(Life {
            life: ship_props.life,
        });
        entity_commands
    }
}

impl WithSprite for Ship {
    fn get_sprite_sheet_bundle(
        handle: &'static str,
        asset_server: Res<AssetServer>,
        texture_atlas_handles: Res<TextureAtlasHandles>,
        texture_atlases: Res<Assets<TextureAtlas>>,
        _props: Option<impl ComponentProps>,
    ) -> Option<SpriteSheetBundle> {
        let atlas_handle = texture_atlas_handles.map.get(&TypeId::of::<TextureAtlas>());
        match atlas_handle {
            Some(ah) => {
                let texture_atlas = texture_atlases.get(ah).unwrap();
                let vendor_handle = asset_server.get_handle(handle);
                let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
                Some(SpriteSheetBundle {
                    transform: Transform {
                        translation: Vec3::new(-300.0, 0.0, 0.0),
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
