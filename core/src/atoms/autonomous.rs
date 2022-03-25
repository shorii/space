use crate::constant::TIME_STEP;
use crate::types::ComponentProps;
use bevy::prelude::*;
use rand::distributions::{Distribution, Standard};
use rand::Rng;

#[derive(Clone)]
pub enum Movement {
    Horizontal,
    Up,
    Down,
}

impl Distribution<Movement> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Movement {
        let value: usize = rng.gen_range(0..=2);
        match value {
            0 => Movement::Horizontal,
            1 => Movement::Up,
            _ => Movement::Down,
        }
    }
}

#[derive(Clone)]
pub enum Direction {
    Right,
    Left,
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
    fn update_transform(&mut self, transform: &mut Transform) {
        let tt = &mut transform.translation;
        let d_direction = match self.direction {
            Direction::Right => 1.0,
            Direction::Left => -1.0,
        };
        tt.x += d_direction * self.speed * TIME_STEP;
        match self.movement {
            Movement::Up => {
                if tt.y >= 380.0 {
                    self.movement = Movement::Down;
                } else {
                    tt.y += 1.0 * self.speed * TIME_STEP;
                }
            }
            Movement::Down => {
                if tt.y <= -380.0 {
                    self.movement = Movement::Up;
                } else {
                    tt.y += -1.0 * self.speed * TIME_STEP;
                }
            }
            _ => {}
        };
    }

    pub fn despawn(commands: &mut Commands, transform: &Transform, entity: Entity) {
        let x = transform.translation.x;
        if x < -640.0 && 640.0 < x {
            commands.entity(entity).despawn();
        }
    }

    pub fn movement_system(
        mut commands: Commands,
        mut query: Query<(Entity, &mut Autonomous, &mut Transform)>,
    ) {
        query.for_each_mut(|autonomous| {
            let (e, mut a, mut transform) = autonomous;
            a.update_transform(&mut transform);
            Self::despawn(&mut commands, &transform, e);
        })
    }
}
