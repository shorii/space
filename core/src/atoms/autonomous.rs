use crate::constant::TIME_STEP;
use crate::types::ComponentProps;
use bevy::prelude::*;

#[derive(Clone)]
pub enum Movement {
    Horizontal,
    Diagonal,
    Random,
}

#[derive(Clone)]
pub struct AutonomousProps {
    pub speed: f32,
    pub movement: Movement,
}

impl ComponentProps for AutonomousProps {}

impl Default for AutonomousProps {
    fn default() -> Self {
        Self {
            speed: 500.0,
            movement: Movement::Horizontal,
        }
    }
}

#[derive(Component)]
pub struct Autonomous {
    pub speed: f32,
    pub movement: Movement,
}

impl Autonomous {
    pub fn movement_system(
        mut commands: Commands,
        mut query: Query<(Entity, &Autonomous, &mut Transform)>,
    ) {
        query.for_each_mut(|autonomous| {
            let (e, b, mut transform) = autonomous;
            let mut direction = 0.0;
            direction += 1.0;
            transform.translation.x += direction * b.speed * TIME_STEP;
            if transform.translation.x > 640.0 {
                commands.entity(e).despawn();
            }
        })
    }
}
