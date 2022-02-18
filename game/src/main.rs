use bevy::core::FixedTimestep;
use bevy::prelude::*;

#[derive(Component)]
struct Ship {
    speed: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct ShootingSpriteHandles {
    handles: Vec<HandleUntyped>,
}

const TIME_STEP: f32 = 1.0 / 60.0;

fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(-200.0, -215.0, 0.0),
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..Default::default()
            },
            sprite: Sprite {
                color: Color::rgb(0.5, 0.5, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Ship { speed: 500.0 });
}

fn ship_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &mut Transform)>,
) {
    let (ship, mut transform) = query.single_mut();
    let mut direction = 0.0;
    if keyboard_input.pressed(KeyCode::Down) {
        direction -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::Up) {
        direction += 1.0;
    };
    transform.translation.y += direction * ship.speed * TIME_STEP;
    transform.translation.y = transform.translation.y.min(380.0).max(-380.0);
}

fn main() {
    App::new()
        .init_resource::<ShootingSpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_state(AppState::Setup)
        .add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ship_movement_system),
        )
        .run();
}

// See following URL
// https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
// https://github.com/bevyengine/bevy/blob/main/examples/game/breakout.rs
