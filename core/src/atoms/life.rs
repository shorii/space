use bevy::prelude::*;

#[derive(Component)]
pub struct Life {
    pub life: u32,
}

impl Life {
    pub fn movement_system<T>(mut commands: Commands, mut query: Query<(Entity, &Life, &T)>)
    where
        T: Component,
    {
        query.for_each_mut(|life| {
            let (e, l, _) = life;
            if l.life == 0 {
                commands.entity(e).despawn();
            }
        })
    }
}
