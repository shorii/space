use crate::atoms::bullet::Bullet;
use crate::constant::TIME_STEP;
use crate::resources::sprite::{SpriteInfo, WithSprite};
use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
}

impl Ship {
    pub fn movement_system(
        mut commands: Commands,
        keyboard_input: Res<Input<KeyCode>>,
        mut query: Query<(&Ship, &mut Transform)>,
    ) {
        match query.get_single_mut() {
            Ok((ship, mut transform)) => {
                let mut direction = 0.0;
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
                    commands
                        .spawn_bundle(SpriteBundle {
                            transform: Transform {
                                translation: fire_point,
                                scale: Vec3::new(10.0, 10.0, 0.0),
                                ..Default::default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(0.5, 0.5, 1.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Bullet {
                            speed: 500.0,
                            distance: 600.0,
                        });
                }
            }
            Err(_) => {
                return;
            }
        }
    }
}

impl WithSprite for Ship {
    fn callback(mut commands: Commands, info: SpriteInfo) {
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(-300.0, 0.0, 0.0),
                    scale: Vec3::splat(0.2),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(info.vendor_index),
                texture_atlas: info.atlas_handle,
                ..Default::default()
            })
            .insert(Ship { speed: 500.0 });
    }
}
