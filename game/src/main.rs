use bevy::asset::LoadState;
use bevy::core::FixedTimestep;
use bevy::prelude::*;

#[derive(Component)]
struct Ship {
    speed: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TextureStartupState {
    Setup,
    Finished,
}

#[derive(Default)]
struct ShootingSpriteHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(
    mut shooting_sprite_handles: ResMut<ShootingSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    shooting_sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

fn check_textures(
    mut state: ResMut<State<TextureStartupState>>,
    shooting_sprite_handles: ResMut<ShootingSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded = asset_server.get_group_load_state(
        shooting_sprite_handles
            .handles
            .iter()
            .map(|handle| handle.id),
    ) {
        state.set(TextureStartupState::Finished).unwrap();
    }
}

fn setup(
    mut commands: Commands,
    shooting_sprite_handles: Res<ShootingSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Image>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in shooting_sprite_handles.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
    }
    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let vendor_handle = asset_server.get_handle("textures/UFO.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(-300.0, 0.0, 0.0),
                scale: Vec3::splat(0.2),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .insert(Ship { speed: 500.0 });
}

const TIME_STEP: f32 = 1.0 / 60.0;

fn ship_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Ship, &mut Transform)>,
) {
    match query.get_single_mut() {
        Ok((ship, mut transform)) => {
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
        Err(_) => {
            return;
        }
    }
}

fn main() {
    App::new()
        .init_resource::<ShootingSpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_state(TextureStartupState::Setup)
        .add_system_set(SystemSet::on_enter(TextureStartupState::Setup).with_system(load_textures))
        .add_system_set(
            SystemSet::on_update(TextureStartupState::Setup).with_system(check_textures),
        )
        .add_system_set(SystemSet::on_enter(TextureStartupState::Finished).with_system(setup))
        .add_system_set(
            SystemSet::on_enter(TextureStartupState::Finished)
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(ship_movement_system),
        )
        .run();
}

// See following URL
// https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs
// https://github.com/bevyengine/bevy/blob/main/examples/game/breakout.rs
