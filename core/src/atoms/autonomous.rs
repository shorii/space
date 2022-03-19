use crate::constant::TIME_STEP;
use crate::types::ComponentProps;
use bevy::prelude::*;

#[derive(Clone)]
pub enum Movement {
    Horizontal,
    Vertical,
}

impl Movement {
    fn increment(&self, autonomous: &Autonomous, transform: &mut Transform) {
        let tt = &mut transform.translation;
        match self {
            Movement::Vertical => {
                let direction = if tt.y >= 380.0 { -1.0 } else { 1.0 };
                tt.y += direction * autonomous.speed * TIME_STEP;
            }
            Movement::Horizontal => {}
        };
    }
}

#[derive(Clone)]
pub enum Direction {
    Right,
    Left,
}

impl Direction {
    fn increment(&self, autonomous: &Autonomous, transform: &mut Transform) {
        let direction = match self {
            Direction::Right => 1.0,
            Direction::Left => -1.0,
        };
        transform.translation.x += direction * autonomous.speed * TIME_STEP;
    }
}

#[derive(Clone)]
pub struct AutonomousProps {
    pub speed: f32,
    pub movement: Movement,
    pub direction: Direction,
}

impl ComponentProps for AutonomousProps {}

#[derive(Component)]
pub struct Autonomous {
    pub speed: f32,
    pub movement: Movement,
    pub direction: Direction,
}

impl Autonomous {
    pub fn despawn(commands: &mut Commands, transform: &Transform, entity: Entity) {
        let x = transform.translation.x;
        if x < -640.0 && 640.0 < x {
            commands.entity(entity).despawn();
        }
    }

    pub fn movement_system(
        mut commands: Commands,
        mut query: Query<(Entity, &Autonomous, &mut Transform)>,
    ) {
        query.for_each_mut(|autonomous| {
            let (e, a, mut transform) = autonomous;
            a.movement.increment(a, &mut transform);
            a.direction.increment(a, &mut transform);
            Self::despawn(&mut commands, &transform, e);
        })
    }
}
