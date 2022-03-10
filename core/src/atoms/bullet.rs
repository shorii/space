use crate::constant::TIME_STEP;
use bevy::prelude::*;

#[derive(Component)]
pub struct Bullet {
    pub speed: f32,
    pub distance: f32,
}

impl Bullet {
    pub fn movement_system(
        mut commands: Commands,
        mut query: Query<(Entity, &Bullet, &mut Transform)>,
    ) {
        query.for_each_mut(|bullet| {
            let (e, b, mut transform) = bullet;
            let mut direction = 0.0;
            direction += 1.0;
            transform.translation.x += direction * b.speed * TIME_STEP;
            if transform.translation.x > 640.0 {
                commands.entity(e).remove::<Bullet>();
            }
        });
    }
}
