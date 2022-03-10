use bevy::asset::LoadState;
use bevy::ecs::schedule::IntoSystemDescriptor;
use bevy::ecs::schedule::SystemDescriptor;
use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StartupState {
    Setup,
    Finished,
}

#[derive(Default)]
pub struct SpriteHandles {
    pub handles: Vec<HandleUntyped>,
}

pub fn load_textures(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

pub fn check_textures(
    mut state: ResMut<State<StartupState>>,
    sprite_handles: ResMut<SpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(StartupState::Finished).unwrap();
    }
}

pub struct SpriteInfo {
    pub vendor_index: usize,
    pub atlas_handle: Handle<TextureAtlas>,
}

pub trait WithSprite {
    fn callback(commands: Commands, info: SpriteInfo) -> ();
}

pub trait WithSpriteExt {
    fn internal_setup_sprite(
        handle: &str,
        sprite_handles: Res<SpriteHandles>,
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut textures: ResMut<Assets<Image>>,
    ) -> SpriteInfo {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
        }
        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let vendor_handle = asset_server.get_handle(handle);
        let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);
        SpriteInfo {
            vendor_index,
            atlas_handle,
        }
    }
    fn setup_sprite(handle: &'static str) -> SystemDescriptor;
}

impl<T> WithSpriteExt for T
where
    T: WithSprite,
{
    fn setup_sprite(handle: &'static str) -> SystemDescriptor {
        let descriptor = Box::new(
            move |mut commands: Commands,
                  sprite_handles: Res<SpriteHandles>,
                  asset_server: Res<AssetServer>,
                  texture_atlases: ResMut<Assets<TextureAtlas>>,
                  textures: ResMut<Assets<Image>>| {
                let sprite_info = Self::internal_setup_sprite(
                    handle,
                    sprite_handles,
                    asset_server,
                    texture_atlases,
                    textures,
                );
                commands.spawn_bundle(OrthographicCameraBundle::new_2d());
                T::callback(commands, sprite_info);
            },
        );
        descriptor.into_descriptor()
    }
}
